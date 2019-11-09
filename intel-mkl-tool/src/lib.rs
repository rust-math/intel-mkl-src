use curl::easy::Easy;
use failure::*;
use log::*;
use std::{
    fs,
    io::{self, Write},
    path::*,
};

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(target_os = "linux")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_linux.tar.xz";
    pub const EXT: &'static str = "so";
    pub const PREFIX: &'static str = "lib";
}

#[cfg(target_os = "macos")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_osx.tar.xz";
    pub const EXT: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
}

#[cfg(target_os = "windows")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_windows64.tar.xz";
    pub const EXT: &'static str = "lib";
    pub const PREFIX: &'static str = "";
}

pub fn download(out_dir: &Path) -> Fallible<()> {
    let archive = out_dir.join(mkl::ARCHIVE);
    if !archive.exists() {
        info!("Download archive from AWS S3: {}/{}", S3_ADDR, mkl::ARCHIVE);
        let f = fs::File::create(&archive)?;
        let mut buf = io::BufWriter::new(f);
        let mut easy = Easy::new();
        easy.url(&format!("{}/{}", S3_ADDR, mkl::ARCHIVE))?;
        easy.write_function(move |data| Ok(buf.write(data).unwrap()))?;
        easy.perform()?;
        assert!(archive.exists());
    } else {
        info!("Use existing archive");
    }

    let core = out_dir.join(format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXT));
    if !core.exists() {
        let f = fs::File::open(&archive)?;
        let de = xz2::read::XzDecoder::new(f);
        let mut arc = tar::Archive::new(de);
        arc.unpack(&out_dir)?;
        assert!(core.exists());
    } else {
        info!("Archive has already been extracted");
    }
    Ok(())
}
