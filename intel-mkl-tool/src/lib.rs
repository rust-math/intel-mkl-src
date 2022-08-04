mod config;
mod entry;

pub use config::*;
pub use entry::*;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod mkl {
    pub const EXTENSION_STATIC: &str = "a";
    pub const EXTENSION_SHARED: &str = "so";
    pub const PREFIX: &str = "lib";
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod mkl {
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const EXTENSION_SHARED: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod mkl {
    pub const EXTENSION_STATIC: &'static str = "lib";
    pub const EXTENSION_SHARED: &'static str = "lib";
    pub const PREFIX: &'static str = "";
}
