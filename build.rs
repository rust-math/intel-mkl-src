// MIT License
//
// Copyright (c) 2017 Toshiki Teramura
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate failure;
extern crate glob;
extern crate pkg_config;
extern crate reqwest;
extern crate tar;
extern crate xz2;

use failure::*;
use glob::glob;
use std::{env, fs, io, path::*};

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

fn main() -> Fallible<()> {
    if pkg_config::find_library("mkl-dynamic-lp64-iomp").is_ok() {
        eprintln!("Returning early, pre-installed Intel mkl was found.");
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let archive = out_dir.join(mkl::ARCHIVE);
    if !archive.exists() {
        eprintln!("Download archive from AWS S3: {}/{}", S3_ADDR, mkl::ARCHIVE);
        let mut res = reqwest::get(&format!("{}/{}", S3_ADDR, mkl::ARCHIVE))?;
        if !res.status().is_success() {
            bail!("HTTP access failed: {}", res.status());
        }
        let f = fs::File::create(&archive)?;
        let mut buf = io::BufWriter::new(f);
        res.copy_to(&mut buf)?;
        assert!(archive.exists());
    } else {
        eprintln!("Use existing archive");
    }

    let core = out_dir.join(format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXT));
    if !core.exists() {
        let f = fs::File::open(&archive)?;
        let de = xz2::read::XzDecoder::new(f);
        let mut arc = tar::Archive::new(de);
        arc.unpack(&out_dir)?;
        assert!(core.exists());
    } else {
        eprintln!("Archive has already been extracted");
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=mkl_intel_thread");
    println!("cargo:rustc-link-lib=mkl_core");
    println!("cargo:rustc-link-lib=iomp5");
    Ok(())
}
