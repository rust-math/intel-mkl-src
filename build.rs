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

extern crate crypto;

use crypto::digest::Digest;
use crypto::md5;

use std::env::var;
use std::fs;
use std::io::*;
use std::path::*;
use std::process::Command;

#[cfg(target_os = "linux")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_linux.tar.xz";
    pub const MD5SUM: &'static str = "03aa6b3918da6046b1225aacd244363a";
    pub const URI: &'static str = "https://www.dropbox.com/s/nnlfdio0ka9yeo1/mkl_linux.tar.xz";
}

#[cfg(target_os = "macos")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_osx.tar.xz";
    pub const MD5SUM: &'static str = "3774e0c8b4ebcb8639a4f293d749bd32";
    pub const URI: &'static str = "https://www.dropbox.com/s/fw74msh8udjdv28/mkl_osx.tar.xz";
}

fn download(uri: &str, filename: &str, out_dir: &Path) {
    let out = out_dir.join(filename);
    let mut f = BufWriter::new(fs::File::create(out).unwrap());
    let p = Command::new("curl")
        .args(&["-L", uri])
        .output()
        .expect("Failed to start download");
    f.write(&p.stdout).unwrap();
}

fn calc_md5(path: &Path) -> String {
    let mut sum = md5::Md5::new();
    let mut f = BufReader::new(fs::File::open(path).unwrap());
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    sum.input(&buf);
    sum.result_str()
}

fn expand(archive: &Path, out_dir: &Path) {
    let st = Command::new("tar")
        .args(&["xvf", archive.to_str().unwrap()])
        .current_dir(&out_dir)
        .status()
        .expect("Failed to start expanding archive");
    if !st.success() {
        panic!("Failed to expand archive");
    }
}

fn main() {
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let archive_path = out_dir.join(mkl::ARCHIVE);

    if archive_path.exists() && calc_md5(&archive_path) == mkl::MD5SUM {
        println!("Use existings archive");
    } else {
        println!("Downlaod archive");
        download(mkl::URI, mkl::ARCHIVE, &out_dir);
        let sum = calc_md5(&archive_path);
        if sum != mkl::MD5SUM {
            panic!(
                "check sum of downloaded archive is incorrect: md5sum={}",
                sum
            );
        }
    }
    expand(&archive_path, &out_dir);

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=mkl_intel_thread");
    println!("cargo:rustc-link-lib=mkl_core");
    println!("cargo:rustc-link-lib=iomp5");
    println!("cargo:rustc-link-lib=m");
}
