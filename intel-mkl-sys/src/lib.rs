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

extern crate intel_mkl_src;

include!("mkl.rs");

// Test linking
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn cos() {
        let a = vec![0.0_f64; 1024];
        let mut b = vec![0.0_f64; 1024];
        unsafe {
            vdCos(1024_i32, a.as_ptr(), b.as_mut_ptr());
        }
    }

    #[test]
    fn new_stream() {
        let mut stream: *mut c_void = std::ptr::null_mut();
        unsafe {
            vslNewStream(
                &mut stream as *mut *mut c_void,
                VSL_BRNG_MT19937 as i32,
                777,
            );
        }
    }
}
