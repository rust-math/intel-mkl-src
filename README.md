# intel-mkl-src
[![Crate](http://meritbadge.herokuapp.com/intel-mkl-src)](https://crates.io/crates/intel-mkl-src)
[![Build Status](https://dev.azure.com/rust-math/intel-mkl-src/_apis/build/status/rust-math.intel-mkl-src?branchName=master)](https://dev.azure.com/rust-math/intel-mkl-src/_build/latest?definitionId=1&branchName=master)

Redistribution of Intel MKL as a crate. Tested on Linux, macOS, and Windows (since 0.4.0)

## Usage

This crate is a `*-src` crate. This downloads and link Intel MKL, but does not introduce any symbols.
Please use `blas-sys`, `lapack-sys`, or `fftw-sys` to use BLAS, LAPACK, FFTW interface of MKL, e.g.

```toml
[dependencies]
fftw-sys = { version = "0.4", features = ["intel-mkl"] }
```

## pkg-config

This crate does not download archive if `pkg-config` finds MKL shared library installed by other way.
Be sure to set `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` correctly.
For debian and ubuntu users, [ci/Dockerfile](ci/Dockerfile) may be helpful.
Windows is not supported yet.

## License
MKL is distributed under the Intel Simplified Software License for Intel(R) Math Kernel Library, See [License.txt](License.txt).
Some wrapper codes are licensed by MIT License (see the header of each file).
