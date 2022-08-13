//! Helper crate of `build.rs` in intel-mkl-src crate.
//!
//! This crate is responsible for finding static or dynamic library
//! of Intel MKL installed in user system.
//!

mod config;
mod entry;

pub use config::*;
pub use entry::*;
