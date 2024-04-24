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

use anyhow::Result;
use intel_mkl_tool::*;
use std::str::FromStr;

macro_rules! def_mkl_config {
    ($cfg:literal) => {
        #[cfg(feature = $cfg)]
        const MKL_CONFIG: &str = $cfg;
    };
}

def_mkl_config!("mkl-static-lp64-iomp");
def_mkl_config!("mkl-static-lp64-seq");
def_mkl_config!("mkl-static-ilp64-iomp");
def_mkl_config!("mkl-static-ilp64-seq");
def_mkl_config!("mkl-dynamic-lp64-iomp");
def_mkl_config!("mkl-dynamic-lp64-seq");
def_mkl_config!("mkl-dynamic-ilp64-iomp");
def_mkl_config!("mkl-dynamic-ilp64-seq");

// Default value
#[cfg(all(
    not(feature = "mkl-static-lp64-iomp"),
    not(feature = "mkl-static-lp64-seq"),
    not(feature = "mkl-static-ilp64-iomp"),
    not(feature = "mkl-static-ilp64-seq"),
    not(feature = "mkl-dynamic-lp64-iomp"),
    not(feature = "mkl-dynamic-lp64-seq"),
    not(feature = "mkl-dynamic-ilp64-iomp"),
    not(feature = "mkl-dynamic-ilp64-seq"),
))]
const MKL_CONFIG: &str = "mkl-static-ilp64-iomp";

fn main() -> Result<()> {
    let cfg = Config::from_str(MKL_CONFIG).unwrap();
    if let Ok(lib) = Library::new(cfg) {
        lib.print_cargo_metadata()?;
        return Ok(());
    }

    // Try ocipkg for static library.
    //
    // This does not work for dynamic library because the directory
    // where ocipkg download archive is not searched by ld
    // unless user set `LD_LIBRARY_PATH` explictly.
    if cfg.link == LinkType::Static {
        if cfg!(target_os = "linux") {
            ocipkg::link_package(&format!(
                "ghcr.io/rust-math/rust-mkl/linux/{}:2020.1-3038006115",
                MKL_CONFIG
            ))?;
        }
        if cfg!(target_os = "windows") {
            ocipkg::link_package(&format!(
                "ghcr.io/rust-math/rust-mkl/windows/{}:2022.0-3038006115",
                MKL_CONFIG
            ))?;
        }
    }

    Ok(())
}
