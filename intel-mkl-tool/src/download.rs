use crate::S3_ADDR;

use anyhow::*;
use curl::easy::Easy;
use log::*;
use std::{fs, path::*};

fn download_archive_to_buffer(url: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.fail_on_error(true)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mkl;

    #[test]
    fn download_url() {
        let url = format!(
            "{}/{}_{}_{}.tar.zst",
            S3_ADDR,
            mkl::ARCHIVE_STATIC,
            mkl::VERSION_YEAR,
            mkl::VERSION_UPDATE
        );
        let _ar = download_archive_to_buffer(&url).unwrap();
    }
}
