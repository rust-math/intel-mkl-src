# intel-mkl-src

|crate         | crate.io                                                                                               | docs.rs                                                                               | master                                                                                                                                    | description                                                           |
|:-------------|:-------------------------------------------------------------------------------------------------------|:--------------------------------------------------------------------------------------|:------------------------------------------------------------------------------------------------------------------------------------------|:----------------------------------------------------------------------|
|intel-mkl-src | [![crate](https://img.shields.io/crates/v/intel-mkl-src.svg)](https://crates.io/crates/intel-mkl-src)  | [![docs.rs](https://docs.rs/intel-mkl-src/badge.svg)](https://docs.rs/intel-mkl-src)  | [![crate](https://img.shields.io/badge/master-intel--mkl--src-blue)](https://rust-math.github.io/intel-mkl-src/intel_mkl_src/index.html)  | Source crate for Intel-MKL                                            |
|intel-mkl-sys | [![Crate](https://img.shields.io/crates/v/intel-mkl-sys.svg)](https://crates.io/crates/intel-mkl-sys)  | [![docs.rs](https://docs.rs/intel-mkl-sys/badge.svg)](https://docs.rs/intel-mkl-sys)  | [![Crate](https://img.shields.io/badge/master-intel--mkl--sys-blue)](https://rust-math.github.io/intel-mkl-src/intel_mkl_sys/index.html)  | FFI for Intel-MKL [vector math][VM], and [statistical functions][VSL] |
|intel-mkl-tool| [![Crate](https://img.shields.io/crates/v/intel-mkl-tool.svg)](https://crates.io/crates/intel-mkl-tool)| [![docs.rs](https://docs.rs/intel-mkl-tool/badge.svg)](https://docs.rs/intel-mkl-tool)| [![Crate](https://img.shields.io/badge/master-intel--mkl--tool-blue)](https://rust-math.github.io/intel-mkl-src/intel_mkl_tool/index.html)| Seek Intel-MKL libraries from filesystem                              |

[VM]:  https://software.intel.com/en-us/mkl-developer-reference-c-vector-mathematical-functions
[VSL]: https://software.intel.com/en-us/mkl-developer-reference-c-statistical-functions

## Usage

`intel-mkl-src` crate is a `*-src` crate. This links MKL libraries to executable build by cargo, but does not provide Rust bindings.
Please use `blas-sys`, `lapack-sys`, or `fftw-sys` to use BLAS, LAPACK, FFTW interface of MKL, e.g.

```toml
[dependencies]
fftw-sys = { version = "0.4", features = ["intel-mkl"] }
```

Binding to MKL specific features are provided by `intel-mkl-sys` crate. This contains 

- [Vector Mathematical Functions](https://www.intel.com/content/www/us/en/develop/documentation/onemkl-developer-reference-c/top/vector-mathematical-functions.html)
- [Statistical Functions](https://www.intel.com/content/www/us/en/develop/documentation/onemkl-developer-reference-c/top/statistical-functions.html)

## How to find system MKL libraries

`intel-mkl-tool` crate seeks system MKL library installed by various installer as following manner:

- Seek using `pkg-config` command
- Seek `${MKLROOT}` directory
- Seek default installation path
  - `/opt/intel/mkl` for Linux
  - `C:/Program Files (x86)/IntelSWTools/` and `C:/Program Files (x86)/Intel/oneAPI` for Windows

If `intel-mkl-tool` does not find MKL library, `intel-mkl-src` try to download MKL binaries from [GitHub Container Registry (ghcr.io)](https://github.com/orgs/rust-math/packages?repo_name=rust-mkl-container).

## Supported features

There are 8 (=2x2x2) `mkl-*-*-*` features to specify how to link MKL libraries.
If any feature is set, default to `mkl-static-ilp64-iomp`.

### Link type (`static` or `dynamic`)
`dynamic` means MKL is linked dynamically, i.e. the executable does not contains MKL libraries
and will seek them from filesystem while execution.
This is better choice when the MKL libraries are managed by the system package manager e.g. `apt`.

`static` means MKL is linked statically, i.e. the MKL binaries are embedded in the executable file.
This is better choice when creating portable executable.

### Data model (`lp64` or `ilp64`)

This specify the data model:

- `ilp64` means `int` (i), `long` (l), and pointers (p) are 64-bit.
- `lp64` means `long` (l) and pointers (p) are 64-bit, `int` is 32-bit.

### Thread management (`iomp` or `seq`)

- `iomp` means MKL uses Intel OpenMP runtime
- `seq` means sequential (single thread) execution

Using GNU OpenMP runtime (`libgomp`) is not supported yet. Please see https://github.com/rust-math/intel-mkl-src/issues/97

## License
MKL is distributed under the Intel Simplified Software License for Intel(R) Math Kernel Library, See [License.txt](License.txt).
Some wrapper codes are licensed by MIT License (see the header of each file).
