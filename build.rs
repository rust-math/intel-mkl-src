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

#[macro_use]
extern crate procedurals;
extern crate curl;

use std::env;
use std::path::*;
use std::process::Command;
use std::fs;
use std::io::*;

use curl::easy::Easy;

const MKL_ARCHIVE: &'static str = "mkl.tar.xz";

#[derive(Debug, EnumError)]
enum Error {
    Io(std::io::Error),
    Curl(curl::Error),
}

type Result<T> = ::std::result::Result<T, Error>;

fn download(uri: &str, filename: &str, out_dir: &Path) -> Result<PathBuf> {
    let out = out_dir.join("filename");
    let mut f = BufWriter::new(fs::File::create(out)?);
    let mut easy = Easy::new();
    easy.url(uri)?;
    easy.write_function(move |data| Ok(f.write(data).unwrap()))?;
    easy.perform()?;
    Ok(out_dir.join(filename).to_path_buf())
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let oid = "fa01d7dfb31f2ebedb59f2654fb85f4c0badce33";
    let uri = format!(
        "https://github.com/termoshtt/rust-intel-mkl/raw/{}/mkl_lib/{}",
        oid,
        MKL_ARCHIVE
    );
    download(&uri, MKL_ARCHIVE, &out_dir).unwrap();
    Command::new("tar")
        .args(&["Jxvf", MKL_ARCHIVE])
        .current_dir(&out_dir)
        .status()
        .expect("Failed to start decompression (maybe 'tar' is missing?)");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=dylib=mkl_gnu_thread");
    println!("cargo:rustc-link-lib=dylib=mkl_core");
    println!("cargo:rustc-link-lib=dylib=gomp");
}
