use crate::*;
use anyhow::*;

/// Handler for found library
#[derive(Debug, Clone, PartialEq)]
pub struct LinkConfig {
    config: Config,
    path: PathBuf,
}

impl LinkConfig {
    pub fn name(&self) -> String {
        self.config.name()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn from_config(config: Config) -> Result<Self> {
        let path = config.base_dir()?;
        Ok(Self { config, path })
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
