use anyhow::{bail, Result};
use intel_mkl_tool::*;
use structopt::StructOpt;

/// CLI tool for intel-mkl crate
#[derive(Debug, StructOpt)]
enum Opt {
    /// Seek Intel-MKL library
    Seek {},
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Seek {} => {
            let available = Entry::available();
            if available.is_empty() {
                bail!("No MKL found");
            }
            for lib in Entry::available() {
                if let Ok(version) = lib.version() {
                    println!("{:<22}: {}.{}", lib.name(), version.0, version.1);
                } else {
                    println!("{:<22}", lib.name());
                }
                for (path, name) in &lib.found_files() {
                    println!("  {:<25} at {}", name, path.display());
                }
            }
        }
    }
    Ok(())
}
