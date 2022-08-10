//! Helper crate of `build.rs` in intel-mkl-src crate.
//!
//! This crate is responsible for setup Intel MKL library
//! usable from Rust crate.
//!
//! - Find library from system.
//! - Download library as a container from OCI registry.
//!

mod config;
mod entry;

pub use config::*;
pub use entry::*;
