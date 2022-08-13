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

#[cfg(feature = "mkl-static-lp64-iomp")]
const MKL_CONFIG: &str = "mkl-static-lp64-iomp";
#[cfg(feature = "mkl-static-lp64-seq")]
const MKL_CONFIG: &str = "mkl-static-lp64-seq";
#[cfg(feature = "mkl-static-ilp64-iomp")]
const MKL_CONFIG: &str = "mkl-static-ilp64-iomp";
#[cfg(feature = "mkl-static-ilp64-seq")]
const MKL_CONFIG: &str = "mkl-static-ilp64-seq";
#[cfg(feature = "mkl-dynamic-lp64-iomp")]
const MKL_CONFIG: &str = "mkl-dynamic-lp64-iomp";
#[cfg(feature = "mkl-dynamic-lp64-seq")]
const MKL_CONFIG: &str = "mkl-dynamic-lp64-seq";
#[cfg(feature = "mkl-dynamic-ilp64-iomp")]
const MKL_CONFIG: &str = "mkl-dynamic-ilp64-iomp";
#[cfg(feature = "mkl-dynamic-ilp64-seq")]
const MKL_CONFIG: &str = "mkl-dynamic-ilp64-seq";

fn main() -> Result<()> {
    let cfg = Config::from_str(MKL_CONFIG).unwrap();
    if let Ok(lib) = Library::new(cfg) {
        lib.print_cargo_metadata()?;
        return Ok(());
    }
    ocipkg::link_package(&format!(
        "ghcr.io/rust-math/rust-mkl/{}:2020.1-2851133947",
        MKL_CONFIG
    ))?;
    Ok(())
}
