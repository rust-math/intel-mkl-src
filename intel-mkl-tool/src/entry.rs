use crate::{Config, LinkType, Threading};
use anyhow::{bail, ensure, Context, Result};
use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::Command,
};

/// Lacked definition of [std::env::consts]
pub const STATIC_EXTENSION: &str = if cfg!(any(target_os = "linux", target_os = "macos")) {
    "a"
} else {
    "lib"
};

/// Found MKL library
///
/// ```no_run
/// use std::str::FromStr;
/// use intel_mkl_tool::{Config, Library};
///
/// let cfg = Config::from_str("mkl-static-lp64-iomp").unwrap();
/// if let Ok(lib) = Library::new(cfg) {
///     lib.print_cargo_metadata().unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Library {
    pub config: Config,
    /// Directory where `mkl.h` and `mkl_version.h` exists
    pub include_dir: PathBuf,
    /// Directory where `libmkl_core.a` or `libmkl_core.so` exists
    pub library_dir: PathBuf,
    /// Directory where `libiomp5.a` or `libiomp5.so` exists
    ///
    /// They are not required for `mkl-*-*-seq` cases,
    /// and then this is `None`.
    pub iomp5_dir: Option<PathBuf>,
}

impl Library {
    /// Find MKL using `pkg-config`
    ///
    /// This only use the installed prefix obtained by `pkg-config --variable=prefix`
    ///
    /// ```text
    /// $ pkg-config --variable=prefix mkl-static-lp64-seq
    /// /opt/intel/mkl
    /// ```
    ///
    /// Then pass it to [Self::seek_directory].
    ///
    /// Limitation
    /// -----------
    /// This will not work for `mkl-*-*-iomp` configure since `libiomp5.{a,so}`
    /// will not be found under the prefix directory of MKL.
    /// Please use `$MKLROOT` environment variable for this case,
    /// see [Self::new] for detail.
    ///
    pub fn pkg_config(config: Config) -> Result<Option<Self>> {
        if let Ok(out) = Command::new("pkg-config")
            .arg("--variable=prefix")
            .arg(config.to_string())
            .output()
        {
            if out.status.success() {
                let path = String::from_utf8(out.stdout).context("Non-UTF8 MKL prefix")?;
                let prefix = Path::new(path.trim());
                let prefix = fs::canonicalize(prefix)?;
                log::info!("pkg-config found {} on {}", config, prefix.display());
                Self::seek_directory(config, prefix)
            } else {
                log::info!("pkg-config does not find {}", config);
                Ok(None)
            }
        } else {
            log::info!("pkg-config itself is not found");
            Ok(None)
        }
    }

    /// Seek MKL libraries in the given directory.
    ///
    /// - This will seek the directory recursively until finding MKL libraries,
    ///   but do not follow symbolic links.
    /// - This will not seek directory named `ia32*`
    /// - Retuns `Ok(None)` if `libiomp5.{a,so}` is not found with `mkl-*-*-iomp` configure
    ///   even if MKL binaries are found.
    ///
    pub fn seek_directory(config: Config, root_dir: impl AsRef<Path>) -> Result<Option<Self>> {
        let root_dir = root_dir.as_ref();
        if !root_dir.is_dir() {
            return Ok(None);
        }
        let mut library_dir = None;
        let mut include_dir = None;
        let mut iomp5_dir = None;
        for path in walkdir::WalkDir::new(root_dir)
            .into_iter()
            .flatten() // skip unreadable directories
            .flat_map(|entry| {
                let path = entry.into_path();
                // Skip directory
                if path.is_dir() {
                    return None;
                }
                // Skip files under directory for ia32
                if path.components().any(|c| {
                    if let std::path::Component::Normal(c) = c {
                        if let Some(c) = c.to_str() {
                            if c.starts_with("ia32") {
                                return true;
                            }
                        }
                    }
                    false
                }) {
                    return None;
                }
                Some(path)
            })
        {
            let dir = path
                .parent()
                .expect("parent must exist here since this is under `root_dir`")
                .to_owned();

            let (stem, ext) = match (path.file_stem(), path.extension()) {
                (Some(stem), Some(ext)) => (
                    stem.to_str().context("Non UTF8 filename")?,
                    ext.to_str().context("Non UTF8 filename")?,
                ),
                _ => continue,
            };

            if stem == "mkl" && ext == "h" {
                log::info!("Found mkl.h: {}", path.display());
                include_dir = Some(dir);
                continue;
            }

            let name = if let Some(name) = stem.strip_prefix(std::env::consts::DLL_PREFIX) {
                name
            } else {
                continue;
            };
            match (config.link, ext) {
                (LinkType::Static, STATIC_EXTENSION) => match name {
                    "mkl_core" => {
                        log::info!("Found: {}", path.display());
                        ensure!(
                            library_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        );
                    }
                    "iomp5" => {
                        log::info!("Found: {}", path.display());
                        ensure!(
                            iomp5_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        );
                    }
                    _ => {}
                },
                (LinkType::Dynamic, std::env::consts::DLL_EXTENSION) => match name {
                    "mkl_core" => {
                        log::info!("Found: {}", path.display());
                        ensure!(
                            library_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        )
                    }
                    "iomp5" => {
                        log::info!("Found: {}", path.display());
                        ensure!(
                            iomp5_dir.replace(dir).is_none(),
                            "Two or more MKL found in {}",
                            root_dir.display()
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if config.parallel == Threading::OpenMP && iomp5_dir.is_none() {
            if let Some(ref lib) = library_dir {
                log::warn!("iomp5 not found while MKL found at {}", lib.display());
            }
            return Ok(None);
        }
        Ok(match (library_dir, include_dir) {
            (Some(library_dir), Some(include_dir)) => Some(Library {
                config,
                include_dir,
                library_dir,
                iomp5_dir,
            }),
            _ => None,
        })
    }

    /// Seek MKL in system
    ///
    /// This try to find installed MKL in following order:
    ///
    /// - Ask to `pkg-config`
    /// - Seek the directory specified by `$MKLROOT` environment variable
    /// - Seek well-known directory
    ///   - `/opt/intel` for Linux
    ///   - `C:/Program Files (x86)/IntelSWTools/` for Windows
    ///
    pub fn new(config: Config) -> Result<Self> {
        if let Some(lib) = Self::pkg_config(config)? {
            return Ok(lib);
        }
        if let Ok(mklroot) = std::env::var("MKLROOT") {
            if let Some(lib) = Self::seek_directory(config, mklroot)? {
                return Ok(lib);
            }
        }
        for path in ["/opt/intel", "C:/Program Files (x86)/IntelSWTools/"] {
            let path = Path::new(path);
            if let Some(lib) = Self::seek_directory(config, path)? {
                return Ok(lib);
            }
        }
        bail!("Intel MKL not found in system");
    }

    pub fn available() -> Vec<Self> {
        Config::possibles()
            .into_iter()
            .flat_map(|cfg| Self::new(cfg).ok())
            .collect()
    }

    /// Found MKL version parsed from `mkl_version.h`
    ///
    /// `mkl_version.h` will define
    ///
    /// ```c
    /// #define __INTEL_MKL__ 2020
    /// #define __INTEL_MKL_MINOR__ 0
    /// #define __INTEL_MKL_UPDATE__ 1
    /// ```
    ///
    /// and this corresponds to `(2020, 0, 1)`
    ///
    pub fn version(&self) -> Result<(u32, u32, u32)> {
        let version_h = self.include_dir.join("mkl_version.h");

        let f = fs::File::open(version_h).context("Failed to open mkl_version.h")?;
        let f = io::BufReader::new(f);
        let mut year = None;
        let mut minor = None;
        let mut update = None;
        for line in f.lines().flatten() {
            if !line.starts_with("#define") {
                continue;
            }
            let ss: Vec<&str> = line.split(' ').collect();
            match ss[1] {
                "__INTEL_MKL__" => year = Some(ss[2].parse()?),
                "__INTEL_MKL_MINOR__" => minor = Some(ss[2].parse()?),
                "__INTEL_MKL_UPDATE__" => update = Some(ss[2].parse()?),
                _ => continue,
            }
        }
        match (year, minor, update) {
            (Some(year), Some(minor), Some(update)) => Ok((year, minor, update)),
            _ => bail!("Invalid mkl_version.h"),
        }
    }

    /// Print `cargo:rustc-link-*` metadata to stdout
    pub fn print_cargo_metadata(&self) -> Result<()> {
        println!("cargo:rustc-link-search={}", self.library_dir.display());
        if let Some(iomp5_dir) = &self.iomp5_dir {
            if iomp5_dir != &self.library_dir {
                println!("cargo:rustc-link-search={}", iomp5_dir.display());
            }
        }
        for lib in self.config.libs() {
            match self.config.link {
                LinkType::Static => {
                    println!("cargo:rustc-link-lib=static={}", lib);
                }
                LinkType::Dynamic => {
                    println!("cargo:rustc-link-lib=dylib={}", lib);
                }
            }
        }
        Ok(())
    }
}
