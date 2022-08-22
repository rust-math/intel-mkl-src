# intel-mkl-src

|crate         | crate.io                                                                                               | docs.rs                                                                               | description                                                           |
|:-------------|:-------------------------------------------------------------------------------------------------------|:--------------------------------------------------------------------------------------|:---------------------------------------------------------------|
|intel-mkl-src | [![crate](https://img.shields.io/crates/v/intel-mkl-src.svg)](https://crates.io/crates/intel-mkl-src)  | [![docs.rs](https://docs.rs/intel-mkl-src/badge.svg)](https://docs.rs/intel-mkl-src)  | Source crate for Intel-MKL                                            |
|intel-mkl-sys | [![Crate](https://img.shields.io/crates/v/intel-mkl-sys.svg)](https://crates.io/crates/intel-mkl-sys)  | [![docs.rs](https://docs.rs/intel-mkl-sys/badge.svg)](https://docs.rs/intel-mkl-sys)  |FFI for Intel-MKL [vector math][VM], and [statistical functions][VSL] |
|intel-mkl-tool| [![Crate](https://img.shields.io/crates/v/intel-mkl-tool.svg)](https://crates.io/crates/intel-mkl-tool)| [![docs.rs](https://docs.rs/intel-mkl-tool/badge.svg)](https://docs.rs/intel-mkl-tool)|Seek Intel-MKL libraries from filesystem                              |

[VM]:  https://software.intel.com/en-us/mkl-developer-reference-c-vector-mathematical-functions
[VSL]: https://software.intel.com/en-us/mkl-developer-reference-c-statistical-functions

## Supported features

`mkl-*-*-*` features specify which MKL to be linked as following.
If any feature is set, default to `mkl-static-ilp64-iomp`.

### Link type (`static` or `dynamic`)
`dynamic` means MKL is linked dynamically, i.e. the executable does not contains MKL libraries
and will seek them from filesystem while execution.
This is better choice when the MKL libraries are managed by the system package manager e.g. `apt`.

`static` means MKL is linked statically, i.e. the MKL binaries are embedded in the executable file.
This is better choice when creating portable executable, or system-managed MKL library does not exist.

### Data model (`lp64` or `ilp64`)

This specify the data model:

- `ilp64` means `int` (i), `long` (l), and pointers (p) are 64-bit.
- `lp64` means `long` (l) and pointers (p) are 64-bit, `int` is 32-bit.

### Thread management (`iomp` or `seq`)

- `iomp` means MKL uses Intel OpenMP runtime
- `seq` means sequential (single thread) execution

Using GNU OpenMP runtime (`libgomp`) is not supported in this project.

## Usage

This crate is a `*-src` crate. This downloads and link Intel MKL, but does not introduce any symbols.
Please use `blas-sys`, `lapack-sys`, or `fftw-sys` to use BLAS, LAPACK, FFTW interface of MKL, e.g.

```toml
[dependencies]
fftw-sys = { version = "0.4", features = ["intel-mkl"] }
```

## How to find system MKL libraries

`intel-mkl-tool` crate seeks system MKL libraries, e.g. installed by various installer as following manner:

- Seek using `pkg-config` command
- Seek `${MKLROOT}` directory
- Seek default installation path
  - `/opt/intel/mkl` for Linux
  - `C:/Program Files (x86)/IntelSWTools/` and `C:/Program Files (x86)/Intel/oneAPI` for Windows

If `intel-mkl-tool` cannot find system MKL, `intel-mkl-src` try to download MKL binaries from OCI Registry.

## License
MKL is distributed under the Intel Simplified Software License for Intel(R) Math Kernel Library, See [License.txt](License.txt).
Some wrapper codes are licensed by MIT License (see the header of each file).
