use anyhow::{bail, Result};
use intel_mkl_tool::Library;
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
            let available = Library::available();
            if available.is_empty() {
                bail!("No MKL found");
            }
            for lib in available {
                let (year, minor, update) = lib.version()?;
                println!("{:<22}: {}.{}.{}", lib.config, year, minor, update);
            }
        }
    }
    Ok(())
}
