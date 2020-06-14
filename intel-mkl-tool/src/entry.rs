use crate::*;
use anyhow::*;
use derive_more::Deref;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deref)]
pub struct Targets(HashMap<String, Option<PathBuf>>);

impl Targets {
    fn new(config: Config) -> Self {
        let mut targets: HashMap<String, Option<PathBuf>> = HashMap::new();
        for name in config.libs() {
            let target = match config.link {
                LinkType::Static => format!("{}{}.{}", mkl::PREFIX, name, mkl::EXTENSION_STATIC),
                LinkType::Shared => format!("{}{}.{}", mkl::PREFIX, name, mkl::EXTENSION_SHARED),
            };
            targets.insert(target, None);
        }
        Self(targets)
    }

    fn found_all(&self) -> bool {
        self.0.iter().all(|(_key, value)| value.is_some())
    }

    fn seek(&mut self, dir: &Path) {
        for (key, value) in &mut self.0 {
            if dir.join(key).exists() {
                value.get_or_insert(dir.canonicalize().unwrap());
            }
        }
    }

    fn paths(&self) -> Vec<PathBuf> {
        let set: HashSet<PathBuf> = self.0.values().cloned().map(|path| path.unwrap()).collect(); // must be redundant
        set.into_iter().collect()
    }
}

/// Handler for found library
#[derive(Debug)]
pub struct Entry {
    config: Config,
    targets: Targets,
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
        if let Ok(lib) = pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(&config.name())
        {
            for path in lib.link_paths {
                targets.seek(&path);
            }

            // assumes following directory structure:
            //
            // - mkl
            //   - include      <- lib.include_paths detects this
            //   - lib/intel64
            for path in lib.include_paths {
                targets.seek(&path.join("../lib/intel64"));
            }
        }

        // XDG_DATA_HOME
        let path = xdg_home_path().join(config.name());
        targets.seek(&path);

        if targets.found_all() {
            return Ok(Self { config, targets });
        } else {
            // None found
            bail!("No library found for {}", config.name());
        }
    }

    pub fn name(&self) -> String {
        self.config.name()
    }

    pub fn targets(&self) -> &Targets {
        &self.targets
    }

    pub fn available() -> Vec<Self> {
        VALID_CONFIGS
            .iter()
            .flat_map(|name| Self::from_config(Config::from_str(name).unwrap()))
            .collect()
    }

    pub fn print_cargo_metadata(&self) {
        for path in self.targets.paths() {
            println!("cargo:rustc-link-search={}", path.display());
        }
        for lib in self.config.libs() {
            match self.config.link {
                LinkType::Static => {
                    println!("cargo:rustc-link-lib=static={}", lib);
                }
                LinkType::Shared => {
                    println!("cargo:rustc-link-lib=shared={}", lib);
                }
            }
        }

        for common in &["pthread", "m", "dl"] {
            println!("cargo:rustc-link-lib=dylib={}", common);
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
