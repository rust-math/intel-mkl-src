use anyhow::{bail, Result};
use intel_mkl_tool::{Config, Library, LinkType, STATIC_EXTENSION};
use oci_spec::image::Platform;
use ocipkg::{image::Builder, ImageName};
use std::{fs, path::Path};

/// Create oci-archive
pub fn pack(cfg: Config, name: &ImageName, output: impl AsRef<Path>) -> Result<()> {
    let lib = Library::new(cfg)?;

    let libs = cfg
        .libs()
        .into_iter()
        .chain(cfg.additional_libs().into_iter())
        .map(|name| {
            let path = lib.library_dir.join(as_library_filename(cfg.link, &name));
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
