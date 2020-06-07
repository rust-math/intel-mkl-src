use crate::mkl;

use anyhow::*;
use glob::glob;
use log::*;
use std::{
    fs,
    io::{self, BufRead},
    path::*,
};

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
        &PathBuf::from(&format!(
            "{}_{}_{}.tar.zst",
            mkl::ARCHIVE_SHARED,
            year,
            update
        )),
    )?;

    if cfg!(target_or = "linux") {
        create_archive(
            &glob(
                mkl_path
                    .join(format!("lib/intel64/*.{}", mkl::EXTENSION_STATIC))
                    .to_str()
                    .unwrap(),
            )?
            .map(|path| path.unwrap())
            .collect::<Vec<_>>(),
            &PathBuf::from(&format!(
                "{}_{}_{}.tar.zst",
                mkl::ARCHIVE_STATIC,
                year,
                update
            )),
        )?;
    }

    Ok(())
}
