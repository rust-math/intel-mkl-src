use anyhow::{bail, Result};
use std::{fmt, str::FromStr};

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

/// How to link MKL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    Static,
    Dynamic,
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkType::Static => write!(f, "static"),
            LinkType::Dynamic => write!(f, "dynamic"),
        }
    }
}

impl Default for LinkType {
    fn default() -> Self {
        LinkType::Static
    }
}

impl FromStr for LinkType {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        Ok(match input {
            "static" => LinkType::Static,
            "dynamic" => LinkType::Dynamic,
            another => bail!("Invalid link spec: {}", another),
        })
    }
}

/// Data model of library
///
/// Array index of some APIs in MKL are defined by `int` in C,
/// whose size is not fixed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataModel {
    /// `long` and pointer are 64bit, i.e. `sizeof(int) == 4`
    LP64,
    /// `int`, `long` and pointer are 64bit, i.e. `sizeof(int) == 8`
    ILP64,
}

impl fmt::Display for DataModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataModel::LP64 => write!(f, "lp64"),
            DataModel::ILP64 => write!(f, "ilp64"),
        }
    }
}

impl Default for DataModel {
    fn default() -> Self {
        DataModel::ILP64
    }
}

impl FromStr for DataModel {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        Ok(match input {
            "lp64" => DataModel::LP64,
            "ilp64" => DataModel::ILP64,
            another => bail!("Invalid index spec: {}", another),
        })
    }
}

/// How to manage thread(s)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Threading {
    /// Use iomp5, Intel OpenMP runtime.
    OpenMP,
    /// No OpenMP runtime.
    Sequential,
}

impl Default for Threading {
    fn default() -> Self {
        Threading::Sequential
    }
}

impl fmt::Display for Threading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Threading::OpenMP => write!(f, "iomp"),
            Threading::Sequential => write!(f, "seq"),
        }
    }
}

impl FromStr for Threading {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        Ok(match input {
            "iomp" => Threading::OpenMP,
            "seq" => Threading::Sequential,
            another => bail!("Invalid parallel spec: {}", another),
        })
    }
}

/// Configuration for Intel MKL, e.g. `mkl-static-lp64-seq`
///
/// There are 2x2x2=8 combinations of [LinkType], [DataModel], and [Threading].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub link: LinkType,
    pub index_size: DataModel,
    pub parallel: Threading,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mkl-{}-{}-{}", self.link, self.index_size, self.parallel)
    }
}

impl FromStr for Config {
    type Err = anyhow::Error;
    fn from_str(name: &str) -> Result<Self> {
        let parts: Vec<_> = name.split('-').collect();
        if parts.len() != 4 {
            bail!("Invalid name: {}", name);
        }
        if parts[0] != "mkl" {
            bail!("Name must start with 'mkl': {}", name);
        }
        Ok(Config {
            link: LinkType::from_str(parts[1])?,
            index_size: DataModel::from_str(parts[2])?,
            parallel: Threading::from_str(parts[3])?,
        })
    }
}

impl Config {
    pub fn possibles() -> Vec<Self> {
        VALID_CONFIGS
            .iter()
            .map(|name| Self::from_str(name).unwrap())
            .collect()
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
                link: LinkType::Static,
                index_size: DataModel::LP64,
                parallel: Threading::OpenMP
            }
        );
        Ok(())
    }

    #[test]
    fn name_to_config_to_name() -> Result<()> {
        for name in VALID_CONFIGS {
            let cfg = Config::from_str(name)?;
            assert_eq!(&cfg.to_string(), name);
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
