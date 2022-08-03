use crate::{mkl, Config, LinkType, VALID_CONFIGS};
use anyhow::{bail, ensure, Context, Result};
use derive_more::Deref;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    str::FromStr,
};

/// A library found in system
#[derive(Debug, Clone)]
pub enum Library {
    PkgConfig {
        config: Config,
        lib: pkg_config::Library,
    },
    Directory {
        config: Config,
        /// Directory where `mkl.h` and `mkl_version.h` exists
        include_dir: PathBuf,
        /// Directory where `libmkl_core.a` or `libmkl_rt.so` exists
        library_dir: PathBuf,
        /// Directory where `libiomp5.a` or `libiomp5.so` exists
        ///
        /// They are sometimes placed in different position.
        /// Returns `None` if they exist on `library_dir`.
        iomp5_dir: Option<PathBuf>,
    },
}

impl Library {
    /// Try to find MKL using pkg-config
    pub fn pkg_config(config: Config) -> Option<Self> {
        if let Ok(lib) = pkg_config::Config::new()
            .cargo_metadata(false)
            .env_metadata(false)
            .probe(&config.to_string())
        {
            Some(Library::PkgConfig { config, lib })
        } else {
            None
        }
    }

    /// Seek MKL libraries in the given directory.
    ///
    /// - This will seek the directory recursively until finding MKL libraries,
    ///   but do not follow symbolic links.
    /// - This will not seek directory named `ia32*`
    ///
    pub fn seek_directory(config: Config, root_dir: impl AsRef<Path>) -> Result<Option<Self>> {
        let root_dir = root_dir.as_ref();
        if !root_dir.is_dir() {
            return Ok(None);
        }
        let mut library_dir = None;
        let mut include_dir = None;
        let mut iomp5_dir = None;
        for entry in walkdir::WalkDir::new(root_dir) {
            let path = entry.unwrap().into_path();
            if path.is_dir() {
                continue;
            }
            let (stem, ext) = match (path.file_stem(), path.extension()) {
                (Some(stem), Some(ext)) => (
                    stem.to_str().context("Non UTF8 filename")?,
                    ext.to_str().context("Non UTF8 filename")?,
                ),
                _ => continue,
            };
            // Skip directory for ia32
            if path.components().any(|c| {
                if let std::path::Component::Normal(c) = c {
                    if let Some(c) = c.to_str() {
                        if c.starts_with("ia32") {
                            return true;
                        }
                    }
                }
                false
            }) {
                continue;
            }

            let dir = path
                .parent()
                .expect("parent must exist here since this is under `root_dir`")
                .to_owned();
            if stem == "mkl" && ext == "h" {
                include_dir = Some(dir);
                continue;
            }
            let name = if let Some(name) = stem.strip_prefix(mkl::PREFIX) {
                name
            } else {
                continue;
            };
            match (config.link, ext) {
                (LinkType::Static, mkl::EXTENSION_STATIC) => match name {
                    "mkl_core" => {
                        ensure!(
                            library_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        )
                    }
                    "iomp5" => {
                        ensure!(
                            iomp5_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        )
                    }
                    _ => {}
                },
                (LinkType::Dynamic, mkl::EXTENSION_SHARED) => match name {
                    "mkl_rt" => {
                        ensure!(
                            library_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        )
                    }
                    "iomp5" => {
                        ensure!(
                            iomp5_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        )
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if library_dir == iomp5_dir {
            iomp5_dir = None;
        }
        Ok(match (library_dir, include_dir) {
            (Some(library_dir), Some(include_dir)) => Some(Library::Directory {
                config,
                include_dir,
                library_dir,
                iomp5_dir,
            }),
            _ => None,
        })
    }

    /// Seek MKL in system
    ///
    /// This try to find installed MKL in following order:
    ///
    /// - Ask to `pkg-config`
    /// - Seek the directory specified by `$MKLROOT` environment variable
    /// - Seek well-known directory
    ///   - `/opt/intel` for Linux
    ///   - `C:/Program Files (x86)/IntelSWTools/` for Windows
    ///
    pub fn new(config: Config) -> Result<Self> {
        if let Some(lib) = Self::pkg_config(config) {
            return Ok(lib);
        }
        if let Ok(mklroot) = std::env::var("MKLROOT") {
            if let Some(lib) = Self::seek_directory(config, mklroot)? {
                return Ok(lib);
            }
        }
        for path in ["/opt/intel", "C:/Program Files (x86)/IntelSWTools/"] {
            let path = Path::new(path);
            if let Some(lib) = Self::seek_directory(config, path)? {
                return Ok(lib);
            }
        }
        bail!("Intel MKL not found in system");
    }

    pub fn config(&self) -> &Config {
        match self {
            Library::PkgConfig { config, .. } => config,
            Library::Directory { config, .. } => config,
        }
    }

    /// Found MKL version parsed from `mkl_version.h`
    ///
    /// `mkl_version.h` will define
    ///
    /// ```c
    /// #define __INTEL_MKL__ 2020
    /// #define __INTEL_MKL_MINOR__ 0
    /// #define __INTEL_MKL_UPDATE__ 1
    /// ```
    ///
    /// and this corresponds to `(2020, 0, 1)`
    ///
    pub fn version(&self) -> Result<(u32, u32, u32)> {
        let version_h = match self {
            Library::PkgConfig { lib, .. } => {
                let mut version_h = None;
                for path in &lib.include_paths {
                    let candidate = path.join("mkl_version.h");
                    if candidate.exists() {
                        version_h = Some(candidate);
                    }
                }
                version_h.context("mkl_version.h not found in pkg-config")?
            }
            Library::Directory { include_dir, .. } => include_dir.join("mkl_version.h"),
        };

        let f = fs::File::open(version_h).context("Failed to open mkl_version.h")?;
        let f = io::BufReader::new(f);
        let mut year = None;
        let mut minor = None;
        let mut update = None;
        for line in f.lines() {
            if let Ok(line) = line {
                if !line.starts_with("#define") {
                    continue;
                }
                let ss: Vec<&str> = line.split(' ').collect();
                match ss[1] {
                    "__INTEL_MKL__" => year = Some(ss[2].parse()?),
                    "__INTEL_MKL_MINOR__" => minor = Some(ss[2].parse()?),
                    "__INTEL_MKL_UPDATE__" => update = Some(ss[2].parse()?),
                    _ => continue,
                }
            }
        }
        match (year, minor, update) {
            (Some(year), Some(minor), Some(update)) => Ok((year, minor, update)),
            _ => bail!("Invalid mkl_version.h"),
        }
    }
}

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
    /// - This will not work for OUT_DIR, or Pkgconfig entry,
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

    /// Seek /opt/intel in Linux system
    #[ignore]
    #[test]
    fn seek_opt_intel() {
        for cfg in Config::possibles() {
            let lib = Library::seek_directory(cfg, "/opt/intel").unwrap().unwrap();
            dbg!(lib.version().unwrap());
        }
    }

    #[ignore]
    #[test]
    fn pkg_config() {
        for cfg in Config::possibles() {
            let lib = Library::pkg_config(cfg).unwrap();
            dbg!(lib.version().unwrap());
        }
    }
}
