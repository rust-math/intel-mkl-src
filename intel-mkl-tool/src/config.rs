use crate::*;
use curl::easy::Easy;
use derive_more::*;
use std::fs;

pub const VALID_CONFIGS: &[&str] = &[
    "mkl-dynamic-ilp64-iomp",
    "mkl-dynamic-ilp64-seq",
    "mkl-dynamic-lp64-iomp",
    "mkl-dynamic-lp64-seq",
    "mkl-static-ilp64-iomp",
    "mkl-static-ilp64-seq",
    "mkl-static-lp64-iomp",
    "mkl-static-lp64-seq",
];

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum LinkType {
    #[display(fmt = "static")]
    Static,
    #[display(fmt = "dynamic")]
    Shared,
}

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum Interface {
    #[display(fmt = "lp64")]
    LP64,
    #[display(fmt = "ilp64")]
    ILP64,
}

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum Threading {
    #[display(fmt = "iomp")]
    OpenMP,
    #[display(fmt = "seq")]
    Sequential,
}

/// Configure for linking, downloading and packaging Intel MKL
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    pub link: LinkType,
    pub index_size: Interface,
    pub parallel: Threading,
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
            "static" => LinkType::Static,
            "dynamic" => LinkType::Shared,
            another => bail!("Invalid link spec: {}", another),
        };

        let index_size = match parts[2] {
            "lp64" => Interface::LP64,
            "ilp64" => Interface::ILP64,
            another => bail!("Invalid index spec: {}", another),
        };

        let parallel = match parts[3] {
            "iomp" => Threading::OpenMP,
            "seq" => Threading::Sequential,
            another => bail!("Invalid parallel spec: {}", another),
        };

        Ok(Config {
            link,
            index_size,
            parallel,
        })
    }

    pub fn possible() -> Vec<Self> {
        VALID_CONFIGS
            .iter()
            .map(|name| Self::from_str(name).unwrap())
            .collect()
    }

    /// identifier used in pkg-config
    pub fn name(&self) -> String {
        format!("mkl-{}-{}-{}", self.link, self.index_size, self.parallel)
    }

    /// Common components
    pub fn libs(&self) -> Vec<String> {
        let mut libs = Vec::new();
        libs.push("mkl_core".into());
        match self.index_size {
            Interface::LP64 => {
                libs.push("mkl_intel_lp64".into());
            }
            Interface::ILP64 => {
                libs.push("mkl_intel_ilp64".into());
            }
        };
        match self.parallel {
            Threading::OpenMP => {
                libs.push("iomp5".into());
                libs.push("mkl_intel_thread".into());
            }
            Threading::Sequential => {
                libs.push("mkl_sequential".into());
            }
        };
        libs
    }

    /// Dynamically loaded libraries, e.g. `libmkl_vml_avx2.so`
    ///
    /// - MKL seeks additional shared library **on runtime**.
    ///   This function lists these files for packaging.
    pub fn additional_libs(&self) -> Vec<String> {
        match self.link {
            LinkType::Static => Vec::new(),
            LinkType::Shared => {
                let mut libs = Vec::new();
                for prefix in &["mkl", "mkl_vml"] {
                    for suffix in &["def", "avx", "avx2", "avx512", "avx512_mic", "mc", "mc3"] {
                        libs.push(format!("{}_{}", prefix, suffix));
                    }
                }
                libs.push("mkl_rt".into());
                libs.push("mkl_vml_mc2".into());
                libs.push("mkl_vml_cmpt".into());
                libs
            }
        }
    }

    /// Download archive from AWS S3, and expand into `${out_dir}/*.so`
    pub fn download<P: AsRef<Path>>(&self, out_dir: P) -> Result<()> {
        let out_dir = out_dir.as_ref();
        if out_dir.exists() {
            fs::create_dir_all(&out_dir)?;
        }
        let data = read_from_url(&format!("{}/{}.tar.zst", s3_addr(), self.name()))?;
        let zstd = zstd::stream::read::Decoder::new(data.as_slice())?;
        let mut arc = tar::Archive::new(zstd);
        arc.unpack(&out_dir)?;
        Ok(())
    }
}

/// Helper for download file from URL
///
/// - This function expands obtained data into memory space
///
fn read_from_url(url: &str) -> Result<Vec<u8>> {
    info!("Downalod {}", url);
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.fail_on_error(true)?;
    handle.url(url)?;
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    Ok(data)
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
                link: LinkType::Static,
                index_size: Interface::LP64,
                parallel: Threading::OpenMP
            }
        );
        Ok(())
    }

    #[test]
    fn name_to_config_to_name() -> Result<()> {
        for name in VALID_CONFIGS {
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
