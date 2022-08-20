use crate::{Config, DataModel, LinkType, Threading};
use anyhow::{bail, Context, Result};
use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::Command,
};

/// MKL Libraries to be linked explicitly,
/// not include OpenMP runtime (iomp5)
pub fn mkl_libs(cfg: Config) -> Vec<String> {
    let mut libs = Vec::new();
    match cfg.index_size {
        DataModel::LP64 => {
            libs.push("mkl_intel_lp64".into());
        }
        DataModel::ILP64 => {
            libs.push("mkl_intel_ilp64".into());
        }
    };
    match cfg.parallel {
        Threading::OpenMP => {
            libs.push("mkl_intel_thread".into());
        }
        Threading::Sequential => {
            libs.push("mkl_sequential".into());
        }
    };
    libs.push("mkl_core".into());
    libs
}

/// MKL Libraries to be loaded dynamically
pub fn mkl_dyn_libs(cfg: Config) -> Vec<String> {
    match cfg.link {
        LinkType::Static => Vec::new(),
        LinkType::Dynamic => {
            let mut libs = Vec::new();
            for prefix in &["mkl", "mkl_vml"] {
                for suffix in &["def", "avx", "avx2", "avx512", "avx512_mic", "mc", "mc3"] {
                    libs.push(format!("{}_{}", prefix, suffix));
                }
            }
            libs.push("mkl_rt".into());
            libs.push("mkl_vml_mc2".into());
            libs.push("mkl_vml_cmpt".into());
            libs
        }
    }
}

/// Filename convention for MKL libraries.
pub fn mkl_file_name(link: LinkType, name: &str) -> String {
    if cfg!(target_os = "windows") {
        match link {
            LinkType::Static => {
                format!("{}.lib", name)
            }
            LinkType::Dynamic => {
                format!("{}_dll.lib", name)
            }
        }
    } else {
        match link {
            LinkType::Static => {
                format!("lib{}.a", name)
            }
            LinkType::Dynamic => {
                format!("lib{}.{}", name, std::env::consts::DLL_EXTENSION)
            }
        }
    }
}

pub const OPENMP_RUNTIME_LIB: &str = if cfg!(target_os = "windows") {
    "iomp5md"
} else {
    "iomp5"
};

/// Filename convention for OpenMP runtime.
pub fn openmp_runtime_file_name(link: LinkType) -> String {
    let name = OPENMP_RUNTIME_LIB;
    if cfg!(target_os = "windows") {
        match link {
            LinkType::Static => {
                format!("lib{}.lib", name)
            }
            LinkType::Dynamic => {
                format!("lib{}.dll", name)
            }
        }
    } else {
        match link {
            LinkType::Static => {
                format!("lib{}.a", name)
            }
            LinkType::Dynamic => {
                format!("lib{}.{}", name, std::env::consts::DLL_EXTENSION)
            }
        }
    }
}

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
        for (dir, file_name) in walkdir::WalkDir::new(root_dir)
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

                let dir = path
                    .parent()
                    .expect("parent must exist here since this is under `root_dir`")
                    .to_owned();

                if let Some(Some(file_name)) = path.file_name().map(|f| f.to_str()) {
                    Some((dir, file_name.to_string()))
                } else {
                    None
                }
            })
        {
            if include_dir.is_none() && file_name == "mkl.h" {
                log::info!("Found mkl.h at {}", dir.display());
                include_dir = Some(dir);
                continue;
            }

            if library_dir.is_none() {
                for name in mkl_libs(config) {
                    if file_name == mkl_file_name(config.link, &name) {
                        log::info!("Found {} at {}", file_name, dir.display());
                        library_dir = Some(dir.clone());
                        continue;
                    }
                }
            }

            // Allow both dynamic/static library by default
            //
            // This is due to some distribution does not provide libiomp5.a
            let possible_link_types = if cfg!(feature = "openmp-strict-link-type") {
                vec![config.link]
            } else {
                vec![config.link, config.link.otherwise()]
            };
            for link in possible_link_types {
                if file_name == openmp_runtime_file_name(link) {
                    log::info!("Found OpenMP runtime ({}): {}", file_name, dir.display());
                    iomp5_dir = Some(dir.clone());
                    continue;
                }
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
    ///   - `C:/Program Files (x86)/IntelSWTools/` and `C:/Program Files (x86)/Intel/oneAPI/` for Windows
    ///
    pub fn new(config: Config) -> Result<Self> {
        if let Some(lib) = Self::pkg_config(config)? {
            return Ok(lib);
        }
        if let Ok(mklroot) = std::env::var("MKLROOT") {
            log::info!("MKLROOT environment variable is detected: {}", mklroot);
            if let Some(lib) = Self::seek_directory(config, mklroot)? {
                return Ok(lib);
            }
        }
        for path in [
            "/opt/intel",
            "C:/Program Files (x86)/IntelSWTools/",
            "C:/Program Files (x86)/Intel/oneAPI/",
        ] {
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
            let ss: Vec<&str> = line.split_whitespace().collect();
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
        println!("cargo:rerun-if-env-changed=MKLROOT");
        println!("cargo:rustc-link-search={}", self.library_dir.display());
        if let Some(iomp5_dir) = &self.iomp5_dir {
            if iomp5_dir != &self.library_dir {
                println!("cargo:rustc-link-search={}", iomp5_dir.display());
            }
        }
        let mut libs = mkl_libs(self.config);
        if self.config.parallel == Threading::OpenMP {
            libs.push(OPENMP_RUNTIME_LIB.to_string());
        }
        for lib in libs {
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
