use derive_more::Display;

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
    fn as_pkg_config_name(&self) -> String {
        format!("mkl-{}-{}-{}", self.link, self.index_size, self.parallel)
    }

    /// Check if pkg-config has a corresponding setting
    pub fn is_pkg_config_managed(&self) -> bool {
        pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(&self.as_pkg_config_name())
            .is_ok()
    }

    /// Check if archive is cached in $XDG_DATA_HOME
    pub fn is_cached(&self) -> bool {
        todo!()
    }

    pub fn print_cargo_metadata(&self) {
        //
    }
}
