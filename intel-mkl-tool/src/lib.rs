use anyhow::*;
use log::*;
use std::path::*;

mod config;
mod entry;

pub use config::*;
pub use entry::*;

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod mkl {
    pub const OS: &str = "linux";
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const EXTENSION_SHARED: &'static str = "so";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2020;
    pub const VERSION_UPDATE: u32 = 1;
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod mkl {
    pub const OS: &str = "macos";
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const EXTENSION_SHARED: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 3;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod mkl {
    pub const OS: &str = "windows";
    pub const EXTENSION_STATIC: &'static str = "lib";
    pub const EXTENSION_SHARED: &'static str = "lib";
    pub const PREFIX: &'static str = "";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}

fn s3_addr() -> String {
    format!(
        "{}/{}/{}.{}",
        S3_ADDR,
        mkl::OS,
        mkl::VERSION_YEAR,
        mkl::VERSION_UPDATE
    )
}

pub fn xdg_home_path() -> PathBuf {
    dirs::data_local_dir().unwrap().join(format!(
        "intel-mkl-tool/{}.{}",
        mkl::VERSION_YEAR,
        mkl::VERSION_UPDATE
    ))
}

pub fn seek_pkg_config() -> Option<PathBuf> {
    if let Ok(lib) = pkg_config::probe_library("mkl-dynamic-lp64-seq") {
        if lib.libs.len() > 1 {
            warn!("Found {} MKL libraries. Use first found.", lib.libs.len())
        }
        return Some(PathBuf::from(lib.libs[0].clone()));
    }
    None
}

pub fn download_default<P: AsRef<Path>>(out_dir: P) -> Result<()> {
    let cfg = Config::from_str("mkl-dynamic-lp64-seq").unwrap();
    cfg.download(out_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download() -> Result<()> {
        download_default("./test_download")?;
        Ok(())
    }
}
