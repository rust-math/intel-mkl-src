use curl::easy::Easy;
use failure::*;
use glob::glob;
use log::*;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::*,
};

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(target_os = "linux")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_linux.tar.xz";
    pub const EXT: &'static str = "so";
    pub const PREFIX: &'static str = "lib";
}

#[cfg(target_os = "macos")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_osx.tar.xz";
    pub const EXT: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
}

#[cfg(target_os = "windows")]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_windows64.tar.xz";
    pub const EXT: &'static str = "lib";
    pub const PREFIX: &'static str = "";
}

pub fn home_library_path() -> PathBuf {
    dirs::data_local_dir().unwrap().join("intel-mkl-tool")
}

pub fn seek_pkg_config() -> Option<PathBuf> {
    if let Ok(lib) = pkg_config::probe_library("mkl-dynamic-lp64-iomp") {
        if lib.libs.len() > 1 {
            warn!("Found {} MKL libraries. Use first found.", lib.libs.len())
        }
        return Some(PathBuf::from(lib.libs[0].clone()));
    }
    None
}

pub fn seek_home() -> Option<PathBuf> {
    let home_lib = home_library_path();
    if home_lib.is_dir() {
        return Some(home_lib);
    }
    None
}

pub fn download(out_dir: &Path) -> Fallible<()> {
    if !out_dir.exists() {
        info!("Create output directory: {}", out_dir.display());
        fs::create_dir_all(out_dir)?;
    }
    if !out_dir.is_dir() {
        bail!("Not a directory: {}", out_dir.display());
    }

    let archive = out_dir.join(mkl::ARCHIVE);
    if !archive.exists() {
        info!("Download archive from AWS S3: {}/{}", S3_ADDR, mkl::ARCHIVE);
        let f = fs::File::create(&archive)?;
        let mut buf = io::BufWriter::new(f);
        let mut easy = Easy::new();
        easy.url(&format!("{}/{}", S3_ADDR, mkl::ARCHIVE))?;
        easy.write_function(move |data| Ok(buf.write(data).unwrap()))?;
        easy.perform()?;
        assert!(archive.exists());
    } else {
        info!("Archive already exists: {}", archive.display());
    }

    let core = out_dir.join(format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXT));
    if !core.exists() {
        let f = fs::File::open(&archive)?;
        let de = xz2::read::XzDecoder::new(f);
        let mut arc = tar::Archive::new(de);
        arc.unpack(&out_dir)?;
        assert!(core.exists());
    } else {
        info!("Archive has already been extracted");
    }
    Ok(())
}

// Read mkl_version.h to get MKL version (e.g. 2019.5)
fn get_mkl_version(version_header: &Path) -> Fallible<(u32, u32)> {
    if !version_header.exists() {
        bail!("MKL Version file not found: {}", version_header.display());
    }
    let f = fs::File::open(version_header)?;
    let f = io::BufReader::new(f);
    let mut year = 0;
    let mut update = 0;
    for line in f.lines() {
        if let Ok(line) = line {
            if !line.starts_with("#define") {
                continue;
            }
            let ss: Vec<&str> = line.split(" ").collect();
            match ss[1] {
                "__INTEL_MKL__" => year = ss[2].parse()?,
                "__INTEL_MKL_UPDATE__" => update = ss[2].parse()?,
                _ => continue,
            }
        }
    }
    if year == 0 || update == 0 {
        bail!("Cannot determine MKL versions");
    }
    Ok((year, update))
}

pub fn package(mkl_path: &Path) -> Fallible<PathBuf> {
    if !mkl_path.exists() {
        bail!("MKL directory not found: {}", mkl_path.display());
    }
    let (year, update) = get_mkl_version(&mkl_path.join("include/mkl_version.h"))?;
    info!("Intel MKL version: {}.{}", year, update);
    let out = if cfg!(target_os = "Linux") {
        let out = PathBuf::from(format!("mkl_linux64_{}_{}.tar.zst", year, update));
        info!("Create archive for Linux/64bit systems: {}", out.display());
        out
    } else {
        let out = PathBuf::from(format!("mkl_windows64_{}_{}.tar.zst", year, update));
        info!(
            "Create archive for Windows/64bit systems: {}",
            out.display()
        );
        out
    };

    let shared_libs: Vec<_> = glob(
        mkl_path
            .join(format!("lib/intel64/*.{}", mkl::EXT))
            .to_str()
            .unwrap(),
    )?
    .map(|path| path.unwrap())
    .collect();
    let f = fs::File::create(&out)?;
    let buf = io::BufWriter::new(f);
    let zstd = zstd::stream::write::Encoder::new(buf, 6)?;
    let mut ar = tar::Builder::new(zstd);
    ar.mode(tar::HeaderMode::Deterministic);
    for lib in &shared_libs {
        info!("Add {}", lib.display());
        ar.append_path_with_name(lib, lib.file_name().unwrap())?;
    }
    let zstd = ar.into_inner()?;
    zstd.finish()?;

    Ok(out)
}
