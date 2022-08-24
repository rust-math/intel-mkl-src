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

### Added

### Changed

### Fixed
- Added path for libiomp5 static libraries under /opt/intel https://github.com/rust-math/intel-mkl-src/pull/63
- make intel-mkl-tool use pkg-config canonically https://github.com/rust-math/intel-mkl-src/pull/65

### Internal

0.7.0+mkl2020.1 - 2022-07-29
=============================

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

0.6.0+mkl2020.1 - 2020-06-23
=============================

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

0.5.0 - 2019-12-15
===================

### Added
- intel-mkl-sys sub-crate for vectorized math and statistiacl functions https://github.com/rust-math/intel-mkl-src/pull/26
- intel-mkl-tool sub-crate and CLI https://github.com/rust-math/intel-mkl-src/pull/20
  - package subcommand https://github.com/rust-math/intel-mkl-src/pull/23

### Changed
- Drop failure dependency https://github.com/rust-math/intel-mkl-src/pull/25
- Use curl instead of reqwest https://github.com/rust-math/intel-mkl-src/pull/19
