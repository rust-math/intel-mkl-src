//! Rust binding to Intel-MKL including
//!
//! - [Vector Mathematical Functions (mkl_vml.h)](https://software.intel.com/en-us/onemkl-developer-reference-c-vector-mathematical-functions)
//! - [Statistical Functions (mkl_vsl.h)](https://software.intel.com/en-us/onemkl-developer-reference-c-statistical-functions)
//!
//! Other parts of Intel-MKL is served via
//!
//! - [blas-sys](https://crates.io/crates/blas-sys)
//! - [lapack-sys](https://crates.io/crates/lapack-sys)
//! - [lapacke-sys](https://crates.io/crates/lapacke-sys)
//! - [fftw-sys](https://crates.io/crates/fftw-sys)
//!
#![allow(
    improper_ctypes,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case
)]
include!("mkl.rs");
