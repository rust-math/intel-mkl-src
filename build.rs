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
use std::path;
use std::process::Command;

fn main() {
    let mkl_dir = path::Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("mkl_lib");
    let st = Command::new("tar")
        .args(&["Jxvf", "mkl.tar.xz"])
        .current_dir(&mkl_dir)
        .status()
        .expect("Failed to start decompression (maybe 'tar' is missing?)");
    if !st.success() {
        panic!("Failed to extract MKL libraries");
    }

    println!("cargo:rustc-link-lib=static=mkl_intel_ilp64");
    println!("cargo:rustc-link-lib=static=mkl_intel_thread");
    println!("cargo:rustc-link-lib=static=iomp5");
    println!("cargo:rustc-link-lib=static=mkl_core");
    println!("cargo:rustc-link-search=native={}", mkl_dir.display());
}
