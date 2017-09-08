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

use std::env;
use std::path::*;
use std::process::Command;

const MKL_ARCHIVE: &'static str = "mkl.tar.xz";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let oid = "7e47b40340b88356058b4cff187ef7598c64658b";
    let uri = format!(
        "https://github.com/termoshtt/rust-intel-mkl/raw/{}/mkl_lib/{}",
        oid,
        MKL_ARCHIVE
    );
    Command::new("wget")
        .args(&["-q", &uri, "-O", MKL_ARCHIVE])
        .current_dir(&out_dir)
        .status()
        .expect("Failed to start download (maybe 'wget' is missing?)");
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
