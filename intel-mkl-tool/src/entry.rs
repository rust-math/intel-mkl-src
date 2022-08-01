use crate::{mkl, xdg_home_path, Config, LinkType, VALID_CONFIGS};
use anyhow::{bail, Result};
use derive_more::Deref;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Deref)]
struct Targets(HashMap<String, Option<PathBuf>>);

impl Targets {
    fn new(config: Config) -> Self {
        let mut targets: HashMap<String, Option<PathBuf>> = HashMap::new();
        for name in config
            .libs()
            .into_iter()
            .chain(config.additional_libs().into_iter())
        {
            let target = match config.link {
                LinkType::Static => format!("{}{}.{}", mkl::PREFIX, name, mkl::EXTENSION_STATIC),
                LinkType::Dynamic => format!("{}{}.{}", mkl::PREFIX, name, mkl::EXTENSION_SHARED),
            };
            targets.insert(target, None);
        }
        Self(targets)
    }

    fn found_files(&self) -> Vec<(PathBuf, String)> {
        self.iter()
            .flat_map(|(name, path)| Some((path.as_ref()?.clone(), name.clone())))
            .collect()
    }

    fn found_any(&self) -> bool {
        self.0.iter().any(|(_key, value)| value.is_some())
    }

    fn seek<P: AsRef<Path>>(&mut self, dir: P) {
        let dir = dir.as_ref();
        for (key, value) in &mut self.0 {
            if dir.join(key).exists() {
                value.get_or_insert(dir.canonicalize().unwrap());
            }
        }
    }
}

#[derive(Debug)]
enum EntryTarget {
    Manual(Targets),
    PkgConfig,
}

/// Handler for found library
#[derive(Debug)]
pub struct Entry {
    config: Config,
    target: EntryTarget,
}

impl Entry {
    /// Get the directory where the library exists
    ///
    /// This will seek followings in this order:
    ///
    /// - `$OUT_DIR`
    ///   - Only for build.rs
    ///   - This exists only when the previous build downloads archive here
    /// - pkg-config `${name}`
    ///   - Installed by package manager or official downloader
    /// - `$XDG_DATA_HOME/intel-mkl-tool/${name}`
    ///   - Downloaded by this crate
    ///
    /// Returns error if no library found
    ///
    pub fn from_config(config: Config) -> Result<Self> {
        let mut targets = Targets::new(config);

        // OUT_DIR
        if let Ok(dir) = std::env::var("OUT_DIR") {
            let out_dir = PathBuf::from(dir);
            targets.seek(&out_dir);
        }

        // pkg-config
        if let Ok(_) = pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(&config.to_string())
        {
            return Ok(Self {
                config,
                target: EntryTarget::PkgConfig,
            });
        }

        // $XDG_DATA_HOME/intel-mkl-tool
        let path = xdg_home_path().join(config.to_string());
        targets.seek(&path);

        // $MKLROOT
        let mkl_root = std::env::var("MKLROOT").map(PathBuf::from);
        if let Ok(path) = mkl_root {
            if path.exists() {
                targets.seek(path.join("lib/intel64"));
            }
        }

        // /opt/intel
        let opt_mkl = PathBuf::from("/opt/intel");
        if opt_mkl.exists() {
            targets.seek(opt_mkl.join("lib/intel64"));
            targets.seek(opt_mkl.join("mkl/lib/intel64"));
        }

        // Default setting for Windows installer
        let windows_mkl =
            PathBuf::from("C:/Program Files (x86)/IntelSWTools/compilers_and_libraries/windows");
        if windows_mkl.exists() {
            targets.seek(windows_mkl.join("mkl/lib/intel64"));
            targets.seek(windows_mkl.join("compiler/lib/intel64"));
        }

        if targets.found_any() {
            Ok(Self {
                config,
                target: EntryTarget::Manual(targets),
            })
        } else {
            // None found
            bail!("No library found for {}", config);
        }
    }

    pub fn name(&self) -> String {
        self.config.to_string()
    }

    pub fn found_files(&self) -> Vec<(PathBuf, String)> {
        if let EntryTarget::Manual(m) = &self.target {
            m.found_files()
        } else {
            vec![]
        }
    }

    pub fn available() -> Vec<Self> {
        VALID_CONFIGS
            .iter()
            .flat_map(|name| Self::from_config(Config::from_str(name).unwrap()))
            .collect()
    }

    /// Get MKL version info from its C header
    ///
    /// - This will not work for OUT_DIR, XDG_DATA_HOME, or Pkgconfig entry,
    ///   and returns Error in these cases
    pub fn version(&self) -> Result<(u32, u32)> {
        for (path, _) in &self.found_files() {
            // assumes following directory structure:
            //
            // - mkl
            //   - include
            //   - lib/intel64 <- this is cached in targets
            //
            let version_header = path.join("../../include/mkl_version.h");
            if !version_header.exists() {
                continue;
            }

            // Extract version info from C header
            //
            // ```
            // #define __INTEL_MKL__ 2020
            // #define __INTEL_MKL_MINOR__ 0
            // #define __INTEL_MKL_UPDATE__ 1
            // ```
            let f = fs::File::open(version_header)?;
            let f = io::BufReader::new(f);
            let mut year = 0;
            let mut update = 0;
            for line in f.lines() {
                if let Ok(line) = line {
                    if !line.starts_with("#define") {
                        continue;
                    }
                    let ss: Vec<&str> = line.split(' ').collect();
                    match ss[1] {
                        "__INTEL_MKL__" => year = ss[2].parse()?,
                        "__INTEL_MKL_UPDATE__" => update = ss[2].parse()?,
                        _ => continue,
                    }
                }
            }
            if year > 0 && update > 0 {
                return Ok((year, update));
            }
        }
        bail!("Cannot determine MKL versions");
    }

    pub fn print_cargo_metadata(&self) {
        match &self.target {
            EntryTarget::Manual(_target) => {
                let paths: HashSet<PathBuf> = self
                    .found_files()
                    .into_iter()
                    .map(|(path, _name)| path)
                    .collect(); // must be redundant
                for path in paths {
                    println!("cargo:rustc-link-search={}", path.display());
                }
                for lib in self.config.libs() {
                    match self.config.link {
                        LinkType::Static => {
                            println!("cargo:rustc-link-lib=static={}", lib);
                        }
                        LinkType::Dynamic => {
                            println!("cargo:rustc-link-lib=dylib={}", lib);
                        }
                    }
                }
            }
            EntryTarget::PkgConfig => {
                pkg_config::Config::new()
                    .probe(&self.config.to_string())
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test all available MKL are detected
    #[ignore]
    #[test]
    fn with_mkl_availables() {
        assert_eq!(Entry::available().len(), 8);
    }
}
