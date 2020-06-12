use anyhow::*;
use derive_more::Display;
use std::path::*;

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum Link {
    #[display(fmt = "static")]
    Static,
    #[display(fmt = "dynamic")]
    Shared,
}

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum IndexSize {
    #[display(fmt = "lp64")]
    LP64,
    #[display(fmt = "ilp64")]
    ILP64,
}

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum Parallel {
    #[display(fmt = "iomp")]
    OpenMP,
    #[display(fmt = "seq")]
    Sequential,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    link: Link,
    index_size: IndexSize,
    parallel: Parallel,
}

impl Config {
    pub fn from_str(name: &str) -> Result<Self> {
        let parts: Vec<_> = name.split("-").collect();
        if parts.len() != 4 {
            bail!("Invalid name: {}", name);
        }

        if parts[0] != "mkl" {
            bail!("Name must start with 'mkl': {}", name);
        }

        let link = match parts[1] {
            "static" => Link::Static,
            "dynamic" => Link::Shared,
            another => bail!("Invalid link spec: {}", another),
        };

        let index_size = match parts[2] {
            "lp64" => IndexSize::LP64,
            "ilp64" => IndexSize::ILP64,
            another => bail!("Invalid index spec: {}", another),
        };

        let parallel = match parts[3] {
            "iomp" => Parallel::OpenMP,
            "seq" => Parallel::Sequential,
            another => bail!("Invalid parallel spec: {}", another),
        };

        Ok(Config {
            link,
            index_size,
            parallel,
        })
    }

    /// identifier used in pkg-config
    pub fn name(&self) -> String {
        format!("mkl-{}-{}-{}", self.link, self.index_size, self.parallel)
    }

    fn base_dir(&self) -> PathBuf {
        todo!()
    }

    /// Static and shared library lists to be linked
    pub fn libs(
        &self,
    ) -> (
        Vec<PathBuf>, /* static */
        Vec<String>,  /* shared */
    ) {
        // FIXME this implementation is for Linux, fix for Windows and macOS
        let mut static_libs = Vec::new();
        let mut shared_libs = vec!["pthread".into(), "m".into(), "dl".into()];

        let mut add = |name: &str| match self.link {
            Link::Static => {
                let base_dir: PathBuf = self.base_dir();
                let path = base_dir.join(format!("lib{}.a", name));
                assert!(path.exists());
                static_libs.push(path);
            }
            Link::Shared => {
                shared_libs.push(name.to_string());
            }
        };

        add("mkl_core");
        match self.index_size {
            IndexSize::LP64 => {
                add("mkl_intel_lp64");
            }
            IndexSize::ILP64 => {
                add("mkl_intel_ilp64");
            }
        };
        match self.parallel {
            Parallel::OpenMP => {
                add("iomp5");
                add("mkl_intel_thread");
            }
            Parallel::Sequential => {
                add("mkl_sequential");
            }
        };
        (static_libs, shared_libs)
    }

    /// Check if pkg-config has a corresponding setting
    pub fn pkg_config(&self) -> Option<pkg_config::Library> {
        pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(&self.name())
            .ok()
    }

    /// Check if archive is cached in $XDG_DATA_HOME
    pub fn exists(&self) -> bool {
        todo!()
    }

    /// Download MKL archive and cache into $XDG_DATA_HOME
    pub fn download(&self) -> PathBuf {
        todo!()
    }

    pub fn print_cargo_metadata(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_to_config() -> Result<()> {
        let cfg = Config::from_str("mkl-static-lp64-iomp")?;
        assert_eq!(
            cfg,
            Config {
                link: Link::Static,
                index_size: IndexSize::LP64,
                parallel: Parallel::OpenMP
            }
        );
        Ok(())
    }

    #[test]
    fn name_to_config_to_name() -> Result<()> {
        let valid_names = [
            "mkl-dynamic-ilp64-iomp",
            "mkl-dynamic-ilp64-seq",
            "mkl-dynamic-lp64-iomp",
            "mkl-dynamic-lp64-seq",
            "mkl-static-ilp64-iomp",
            "mkl-static-ilp64-seq",
            "mkl-static-lp64-iomp",
            "mkl-static-lp64-seq",
        ];
        for name in &valid_names {
            let cfg = Config::from_str(name)?;
            assert_eq!(&cfg.name(), name);
        }
        Ok(())
    }

    #[test]
    fn invalid_names() -> Result<()> {
        assert!(Config::from_str("").is_err());
        assert!(Config::from_str("static-lp64-iomp").is_err());
        assert!(Config::from_str("mkll-static-lp64-iomp").is_err());
        assert!(Config::from_str("mkl-sttic-lp64-iomp").is_err());
        assert!(Config::from_str("mkl-static-l64-iomp").is_err());
        assert!(Config::from_str("mkl-static-lp64-omp").is_err());
        Ok(())
    }
}
