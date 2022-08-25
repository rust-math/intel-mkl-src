# Changelog

- All notable changes to this project will be documented in this file.
  - The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
  - and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

In addition to original Keep-a-Changelog, we use following rules:

- Use [GitHub Flavored Markdown](https://github.github.com/gfm/)
- Each line in changes SHOULD include a link to Pull Request in GitHub
- Each Pull Request MUST add a line in this file
  - This will be checked by GitHub Actions
- Each Pull Request MAY correspond to one or more lines in this file

## Unreleased

### Fixed
- Remove `use *` for external crates https://github.com/rust-math/intel-mkl-src/pull/70
- Added path for libiomp5 static libraries under /opt/intel https://github.com/rust-math/intel-mkl-src/pull/63
- make intel-mkl-tool use pkg-config canonically https://github.com/rust-math/intel-mkl-src/pull/65

### Changed
- Remove default feature, use mkl-static-ilp64-iomp if no feature https://github.com/rust-math/intel-mkl-src/pull/95
- `intel_mkl_tool::Entry` is rewritten into `intel_mkl_tool::Library` https://github.com/rust-math/intel-mkl-src/pull/81
  - Executable for seeking MKL in system as an example (alternetive to intel-mkl-tool cli) https://github.com/rust-math/intel-mkl-src/pull/92
  - Skip unreadable directory https://github.com/rust-math/intel-mkl-src/pull/91
  - `intel-mkl-tool::Library` supports windows https://github.com/rust-math/intel-mkl-src/pull/90
    - Support `mkl-dynamic-*-*` cases for Windows https://github.com/rust-math/intel-mkl-src/pull/94
- Rename `intel_mkl_tool::Interface` to `DataModel`, `LinkType::Shared` to `Dynamic` https://github.com/rust-math/intel-mkl-src/pull/79
- Minimal supported rustc version to 1.56.0, dirs 4.0 https://github.com/rust-math/intel-mkl-src/pull/73 https://github.com/rust-math/intel-mkl-src/pull/74
- zstd version range `<=0.11, >=0.6` https://github.com/rust-math/intel-mkl-src/pull/71

### Added
- Try ocipkg when MKL not found https://github.com/rust-math/intel-mkl-src/pull/88

### Removed
- Split container management as another repository https://github.com/rust-math/rust-mkl-container
  - Drop intel-mkl-pack https://github.com/rust-math/intel-mkl-src/pull/87
  - Separate container management https://github.com/rust-math/intel-mkl-src/pull/86
  - Create new archive using ocipkg https://github.com/rust-math/intel-mkl-src/pull/84
  - Add patch version in container https://github.com/rust-math/intel-mkl-src/pull/83
- Drop `download` feature https://github.com/rust-math/intel-mkl-src/pull/82
- Drop `download` from default feature https://github.com/rust-math/intel-mkl-src/pull/75
- Remove `xdg-data-home` experimental feature https://github.com/rust-math/intel-mkl-src/pull/80

### Internal
- Additional CHANGELOG rule #99 https://github.com/rust-math/intel-mkl-src/pull/99
- Deploy cargo-doc to GitHub Pages https://github.com/rust-math/intel-mkl-src/pull/98
- Update README https://github.com/rust-math/intel-mkl-src/pull/96
- Test case for seeking MKL installed by apt https://github.com/rust-math/intel-mkl-src/pull/93
- clippy fix https://github.com/rust-math/intel-mkl-src/pull/89
- Regenerate intel-mkl-sys FFI using bindgen 0.60.1 https://github.com/rust-math/intel-mkl-src/pull/78

## 0.7.0+mkl2020.1 - 2022-07-29

Released 2 crates

- intel-mkl-src 0.7.0+mkl2020.1
- intel-mkl-tool 0.3.0+mkl2020.1

### Fixed
- Remove use * for external crates for anyhow error https://github.com/rust-math/intel-mkl-src/pull/70

### Changed
- Set minimal supported rustc version (MSRV) to 1.56.0 https://github.com/rust-math/intel-mkl-src/pull/73
- Update dependencies https://github.com/rust-math/intel-mkl-src/pull/74
  - zstd 0.11 https://github.com/rust-math/intel-mkl-src/pull/71
  - dirs 4.0 https://github.com/rust-math/intel-mkl-src/pull/74
- Repository of container image has been moved to GitHub Container Registry (ghcr.io) from DockerHub https://github.com/rust-math/intel-mkl-src/pull/60

## 0.6.0+mkl2020.1 - 2020-06-23

Released 3 crates

- intel-mkl-src 0.6.0+mkl2020.1
- intel-mkl-sys 0.2.0+mkl2020.1
- intel-mkl-tool 0.2.0+mkl2020.1

### Added

- Static link support https://github.com/rust-math/intel-mkl-src/issues/30
  - For Windows https://github.com/rust-math/intel-mkl-src/pull/48
  - For Linux https://github.com/rust-math/intel-mkl-src/pull/45

### Changed
- Add MKL version into crate version https://github.com/rust-math/intel-mkl-src/pull/50
- Based on Intel MKL 2020.1
  - For Linux https://github.com/rust-math/intel-mkl-src/pull/43
  - For Windows https://github.com/rust-math/intel-mkl-src/pull/48
- Refactoring intel-mkl-tool
  - Switch failure to anyhow https://github.com/rust-math/intel-mkl-src/pull/33
  - and others...

### Deleted
- macOS support is dropped https://github.com/rust-math/intel-mkl-src/issues/42

### Maintenance
- Create MKL-enable Rust container https://github.com/rust-math/intel-mkl-src/pull/36
- Switch to GitHub Actions https://github.com/rust-math/intel-mkl-src/pull/32

## 0.5.0 - 2019-12-15

### Added
- intel-mkl-sys sub-crate for vectorized math and statistiacl functions https://github.com/rust-math/intel-mkl-src/pull/26
- intel-mkl-tool sub-crate and CLI https://github.com/rust-math/intel-mkl-src/pull/20
  - package subcommand https://github.com/rust-math/intel-mkl-src/pull/23

### Changed
- Drop failure dependency https://github.com/rust-math/intel-mkl-src/pull/25
- Use curl instead of reqwest https://github.com/rust-math/intel-mkl-src/pull/19
