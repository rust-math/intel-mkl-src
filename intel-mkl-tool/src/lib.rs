use anyhow::*;
use curl::easy::Easy;
use glob::glob;
use log::*;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::*,
};

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_linux64";
    pub const EXT: &'static str = "so";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_macos64";
    pub const EXT: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 3;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod mkl {
    pub const ARCHIVE: &'static str = "mkl_windows64";
    pub const EXT: &'static str = "lib";
    pub const PREFIX: &'static str = "";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}

pub fn archive_filename(archive: &str, year: u32, update: u32) -> String {
    format!("{}_{}_{}.tar.zst", archive, year, update)
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

pub fn download(out_dir: &Path) -> Result<()> {
    if !out_dir.exists() {
        info!("Create output directory: {}", out_dir.display());
        fs::create_dir_all(out_dir)?;
    }
    if !out_dir.is_dir() {
        bail!("Not a directory: {}", out_dir.display());
    }

    let filename = archive_filename(mkl::ARCHIVE, mkl::VERSION_YEAR, mkl::VERSION_UPDATE);
    let archive = out_dir.join(&filename);
    if !archive.exists() {
        let url = format!("{}/{}", S3_ADDR, filename);
        info!("Download archive from AWS S3: {}", url);
        let f = fs::File::create(&archive)?;
        let mut buf = io::BufWriter::new(f);
        let mut easy = Easy::new();
        easy.url(&url)?;
        easy.write_function(move |data| Ok(buf.write(data).unwrap()))?;
        easy.perform()?;
        assert!(archive.exists());
    } else {
        info!("Archive already exists: {}", archive.display());
    }

    let core = out_dir.join(format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXT));
    if !core.exists() {
        let f = fs::File::open(&archive)?;
        let de = zstd::stream::read::Decoder::new(f)?;
        let mut arc = tar::Archive::new(de);
        arc.unpack(&out_dir)?;
        assert!(core.exists());
    } else {
        info!("Archive has already been extracted");
    }
    Ok(())
}

// Read mkl_version.h to get MKL version (e.g. 2019.5)
fn get_mkl_version(version_header: &Path) -> Result<(u32, u32)> {
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

pub fn package(mkl_path: &Path) -> Result<PathBuf> {
    if !mkl_path.exists() {
        bail!("MKL directory not found: {}", mkl_path.display());
    }
    let (year, update) = get_mkl_version(&mkl_path.join("include/mkl_version.h"))?;
    info!("Intel MKL version: {}.{}", year, update);
    let out = PathBuf::from(archive_filename(mkl::ARCHIVE, year, update));
    info!("Create archive: {}", out.display());

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
