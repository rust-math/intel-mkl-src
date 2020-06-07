use anyhow::*;
use curl::easy::Easy;
use glob::glob;
use log::*;
use std::{
    fs,
    io::{self, BufRead},
    path::*,
};

const S3_ADDR: &'static str = "https://s3-ap-northeast-1.amazonaws.com/rust-intel-mkl";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod mkl {
    pub const ARCHIVE_SHARED: &'static str = "mkl_linux64_shared";
    pub const ARCHIVE_STATIC: &'static str = "mkl_linux64_static";
    pub const EXTENSION_SHARED: &'static str = "so";
    pub const EXTENSION_STATIC: &'static str = "a";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2020;
    pub const VERSION_UPDATE: u32 = 1;
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub mod mkl {
    pub const ARCHIVE_SHARED: &'static str = "mkl_macos64_shared";
    pub const ARCHIVE_STATIC: &'static str = "mkl_macos64_static";
    pub const EXTENSION_SHARED: &'static str = "dylib";
    pub const PREFIX: &'static str = "lib";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 3;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub mod mkl {
    pub const ARCHIVE_SHARED: &'static str = "mkl_windows64";
    pub const EXTENSION_SHARED: &'static str = "lib";
    pub const PREFIX: &'static str = "";
    pub const VERSION_YEAR: u32 = 2019;
    pub const VERSION_UPDATE: u32 = 5;
}

pub fn archive_filename(archive: &str, year: u32, update: u32) -> String {
    format!("{}_{}_{}.tar.zst", archive, year, update)
}

pub fn xdg_home_path() -> PathBuf {
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
    let home_lib = xdg_home_path();
    if home_lib.is_dir() {
        return Some(home_lib);
    }
    None
}

fn download_archive_to_buffer(url: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url)?;
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    Ok(data)
}

pub fn download(base_dir: &Path, prefix: &str, year: u32, update: u32) -> Result<()> {
    let filename = format!("{}_{}_{}.tar.zst", prefix, year, update);
    let dest_dir = base_dir.join(&format!("{}_{}_{}", prefix, year, update));

    if dest_dir.exists() {
        bail!("Directory already exists: {}", dest_dir.display());
    }
    fs::create_dir_all(&dest_dir)?;

    info!("Download archive {} into {}", filename, dest_dir.display());
    let data = download_archive_to_buffer(&format!("{}/{}", S3_ADDR, filename))?;
    let zstd = zstd::stream::read::Decoder::new(data.as_slice())?;
    let mut arc = tar::Archive::new(zstd);
    arc.unpack(&dest_dir)?;
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

// Create tar.zst archive from path list
fn create_archive(libs: &[PathBuf], out: &Path) -> Result<()> {
    if out.exists() {
        bail!("Output archive already exits: {}", out.display());
    }
    info!("Create archive: {}", out.display());
    let f = fs::File::create(&out)?;
    let buf = io::BufWriter::new(f);
    let zstd = zstd::stream::write::Encoder::new(buf, 6)?;
    let mut ar = tar::Builder::new(zstd);
    ar.mode(tar::HeaderMode::Deterministic);
    for lib in libs {
        info!("Add {}", lib.display());
        ar.append_path_with_name(lib, lib.file_name().unwrap())?;
    }
    let zstd = ar.into_inner()?;
    zstd.finish()?;
    Ok(())
}

pub fn package(mkl_path: &Path) -> Result<()> {
    if !mkl_path.exists() {
        bail!("MKL directory not found: {}", mkl_path.display());
    }
    let (year, update) = get_mkl_version(&mkl_path.join("include/mkl_version.h"))?;
    info!("Intel MKL version: {}.{}", year, update);

    create_archive(
        &glob(
            mkl_path
                .join(format!("lib/intel64/*.{}", mkl::EXTENSION_SHARED))
                .to_str()
                .unwrap(),
        )?
        .map(|path| path.unwrap())
        .collect::<Vec<_>>(),
        &PathBuf::from(archive_filename(mkl::ARCHIVE_SHARED, year, update)),
    )?;

    create_archive(
        &glob(
            mkl_path
                .join(format!("lib/intel64/*.{}", mkl::EXTENSION_STATIC))
                .to_str()
                .unwrap(),
        )?
        .map(|path| path.unwrap())
        .collect::<Vec<_>>(),
        &PathBuf::from(archive_filename(mkl::ARCHIVE_STATIC, year, update)),
    )?;

    Ok(())
}
