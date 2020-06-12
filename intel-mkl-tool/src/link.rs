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
                } else {
                    warn!("{} not found in {}", &core, path.display());
                }
            } else if !lib.include_paths.is_empty() {
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
            } else {
                warn!(
                    "No link path exists in pkg-config entry of {}",
                    config.name()
                );
            }
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

    pub fn print_cargo_metadata(&self) -> Result<()> {
        let (_static, _shared) = self.libs()?;
        todo!()
    }

    /// Static and shared library lists to be linked
    fn libs(&self) -> Result<(Vec<PathBuf>, Vec<String>)> {
        // FIXME this implementation is for Linux, fix for Windows and macOS
        let mut static_libs = Vec::new();
        let mut shared_libs = vec!["pthread".into(), "m".into(), "dl".into()];

        let mut add = |name: &str| match self.config.link {
            Link::Static => {
                let path = self.path.join(format!("lib{}.a", name));
                assert!(path.exists());
                static_libs.push(path);
            }
            Link::Shared => {
                shared_libs.push(name.to_string());
            }
        };

        add("mkl_core");
        match self.config.index_size {
            IndexSize::LP64 => {
                add("mkl_intel_lp64");
            }
            IndexSize::ILP64 => {
                add("mkl_intel_ilp64");
            }
        };
        match self.config.parallel {
            Parallel::OpenMP => {
                add("iomp5");
                add("mkl_intel_thread");
            }
            Parallel::Sequential => {
                add("mkl_sequential");
            }
        };
        Ok((static_libs, shared_libs))
    }
}
