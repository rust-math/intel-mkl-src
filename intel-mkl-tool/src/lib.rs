use log::*;
use std::path::*;

mod config;
mod download;
mod link;
mod package;

pub use config::*;
pub use download::*;
pub use link::*;
pub use package::*;

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_linux64";
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const EXTENSION_SHARED: &'static str = "so";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_macos64";
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const EXTENSION_SHARED: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 3;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_windows64";
    pub const EXTENSION_STATIC: &'static str = "lib";
    pub const EXTENSION_SHARED: &'static str = "lib";
    pub const PREFIX: &'static str = "";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}

pub fn archive_filename() -> String {
    format!(
        "{}_{}_{}.tar.zst",
        mkl::ARCHIVE,
        mkl::VERSION_YEAR,
        mkl::VERSION_UPDATE
    )
}

pub fn xdg_home_path() -> PathBuf {
    dirs::data_local_dir().unwrap().join("intel-mkl-tool")
}

pub fn seek_pkg_config() -> Option<PathBuf> {
    if let Ok(lib) = pkg_config::probe_library("mkl-dynamic-lp64-iomp") {
        if lib.libs.len() > 1 {
            warn!("Found {} MKL libraries. Use first found.", lib.libs.len())
        }
        return Some(PathBuf::from(lib.libs[0].clone()));
    }
    None
}

pub fn seek_home() -> Option<PathBuf> {
    let home_lib = xdg_home_path();
    if home_lib.is_dir() {
        return Some(home_lib);
    }
    None
}
