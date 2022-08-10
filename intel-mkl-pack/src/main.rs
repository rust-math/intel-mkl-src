//! Create container of MKL library, found by intel-mkl-tool

use anyhow::{bail, Result};
use colored::Colorize;
use intel_mkl_tool::{Config, Library, LinkType, STATIC_EXTENSION};
use oci_spec::image::Platform;
use ocipkg::{image::Builder, ImageName};
use std::{fs, path::Path, time::Instant};

const REGISTRY: &str = "ghcr.io/rust-math/intel-mkl-src";

fn main() -> Result<()> {
    let run_id: u64 = std::env::var("GITHUB_RUN_ID")
        .unwrap_or("0".to_string()) // fallback value for local testing
        .parse()?;
    for cfg in Config::possibles() {
        let lib = Library::new(cfg)?;
        let (year, _, update) = lib.version()?;
        let name = ImageName::parse(&format!(
            "{}/{}:{}.{}-{}",
            REGISTRY, cfg, year, update, run_id
        ))?;
        let output = format!("{}.tar", cfg);

        eprintln!("{:>12} {}", "Packaging".green().bold(), name);
        let timer = Instant::now();
        pack(cfg, &name, &output)?;
        eprintln!(
            "{:>12} {} ({:.2}s)",
            "Created".green().bold(),
            output,
            timer.elapsed().as_secs_f32()
        );
    }
    Ok(())
}

/// Create oci-archive
pub fn pack(cfg: Config, name: &ImageName, output: impl AsRef<Path>) -> Result<()> {
    let lib = Library::new(cfg)?;

    let libs = cfg
        .libs()
        .into_iter()
        .chain(cfg.additional_libs().into_iter())
        .map(|name| {
            let path = if name == "iomp5" {
                lib.iomp5_dir
                    .as_ref()
                    .unwrap()
                    .join(as_library_filename(cfg.link, &name))
            } else {
                lib.library_dir.join(as_library_filename(cfg.link, &name))
            };
            if !path.exists() {
                bail!("Required library not found: {}", path.display());
            }
            Ok(path)
        })
        .collect::<Result<Vec<_>>>()?;

    let mut f = fs::File::create(output)?;
    let mut builder = Builder::new(&mut f);
    builder.append_files(&libs)?;
    builder.set_platform(&Platform::default());
    builder.set_name(&name);
    Ok(())
}

fn as_library_filename(link: LinkType, name: &str) -> String {
    match link {
        LinkType::Static => format!(
            "{}{}.{}",
            std::env::consts::DLL_PREFIX,
            name,
            STATIC_EXTENSION
        ),
        LinkType::Dynamic => format!(
            "{}{}.{}",
            std::env::consts::DLL_PREFIX,
            name,
            std::env::consts::DLL_EXTENSION
        ),
    }
}
