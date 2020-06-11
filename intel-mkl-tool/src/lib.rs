mod config;
mod download;
mod package;
mod seek;

pub use config::*;
pub use download::*;
pub use package::*;
pub use seek::*;

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod mkl {
    pub const ARCHIVE_SHARED: &'static str = "mkl_linux64_shared";
    pub const ARCHIVE_STATIC: &'static str = "mkl_linux64_static";
    pub const EXTENSION_SHARED: &'static str = "so";
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2020;
    pub const VERSION_UPDATE: u32 = 1;
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub mod mkl {
    pub const ARCHIVE_SHARED: &'static str = "mkl_macos64_shared";
    pub const EXTENSION_SHARED: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 3;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub mod mkl {
    pub const ARCHIVE_SHARED: &'static str = "mkl_windows64";
    pub const EXTENSION_SHARED: &'static str = "lib";
    pub const PREFIX: &'static str = "";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}
