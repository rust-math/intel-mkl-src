use crate::{mkl, S3_ADDR};

use anyhow::*;
use curl::easy::Easy;
use log::*;
use std::path::*;

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

pub fn download(out_dir: &Path, prefix: &str, year: u32, update: u32) -> Result<()> {
    let mkl_core = out_dir.join(format!("{}mkl_core.{}", mkl::PREFIX, mkl::EXT));
    if mkl_core.exists() {
        info!("Archive already exists: {}", out_dir.display());
        return Ok(());
    }

    let filename = format!("{}_{}_{}.tar.zst", prefix, year, update);
    info!("Download archive {} into {}", filename, out_dir.display());
    let data = download_archive_to_buffer(&format!("{}/{}", S3_ADDR, filename))?;
    let zstd = zstd::stream::read::Decoder::new(data.as_slice())?;
    let mut arc = tar::Archive::new(zstd);
    arc.unpack(&out_dir)?;
    Ok(())
}

pub fn download_default(out_dir: &Path) -> Result<()> {
    download(
        out_dir,
        mkl::ARCHIVE,
        mkl::VERSION_YEAR,
        mkl::VERSION_UPDATE,
    )
}
