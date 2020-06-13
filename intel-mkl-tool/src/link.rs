use crate::*;
use anyhow::*;

/// Handler for found library
#[derive(Debug, Clone, PartialEq)]
pub struct LinkConfig {
    config: Config,
    path: PathBuf,
}

impl LinkConfig {
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
        let core = match config.link {
            Link::Static => format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXTENSION_STATIC),
            Link::Shared => format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXTENSION_SHARED),
        };

        // OUT_DIR
        if let Ok(dir) = std::env::var("OUT_DIR") {
            let out_dir = PathBuf::from(dir);
            if out_dir.join(&core).exists() {
                return Ok(Self {
                    config,
                    path: out_dir,
                });
            }
        }

        // pkg-config
        if let Ok(lib) = pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(&config.name())
        {
            if !lib.link_paths.is_empty() {
                let path = &lib.link_paths[0];
                if path.join(&core).exists() {
                    return Ok(Self {
                        config,
                        path: path.clone(),
                    });
                }
            }
            if !lib.include_paths.is_empty() {
                // assumes following directory structure:
                //
                // - mkl
                //   - include      <- lib.include_paths detects this
                //   - lib/intel64
                let path = lib.include_paths[0].join("../lib/intel64");
                if path.join(&core).exists() {
                    return Ok(Self {
                        config,
                        path: path.canonicalize()?.clone(),
                    });
                }
            }
            warn!(
                "No link path exists in pkg-config entry of {}",
                config.name()
            );
        }

        // XDG_DATA_HOME
        let path = xdg_home_path().join(config.name());
        if path.exists() && path.join(&core).exists() {
            return Ok(Self { config, path });
        }

        // None found
        bail!("No library found for {}", config.name());
    }

    pub fn name(&self) -> String {
        self.config.name()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn available() -> Vec<Self> {
        VALID_CONFIGS
            .iter()
            .flat_map(|name| Self::from_config(Config::from_str(name).unwrap()))
            .collect()
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search={}", self.path.display());
        for lib in self.config.libs() {
            match self.config.link {
                Link::Static => {
                    let path =
                        self.path
                            .join(format!("{}{}.{}", mkl::PREFIX, lib, mkl::EXTENSION_STATIC));
                    if !path.exists() {
                        panic!("Static library not found: {}", path.display());
                    }
                    println!("cargo:rustc-link-lib=static={}", lib);
                }
                Link::Shared => {
                    let path =
                        self.path
                            .join(format!("{}{}.{}", mkl::PREFIX, lib, mkl::EXTENSION_SHARED));
                    if !path.exists() {
                        panic!("Shared library not found: {}", path.display());
                    }
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
        assert_eq!(LinkConfig::available().len(), 8);
    }

    #[ignore]
    #[test]
    fn with_mkl_print_cargo_metadata() {
        for cfg in LinkConfig::available() {
            // check asserts
            cfg.print_cargo_metadata();
        }
    }
}
