use anyhow::*;
use intel_mkl_tool::*;
use std::{env, path::PathBuf};
use structopt::StructOpt;

/// CLI tool for intel-mkl crate
#[derive(Debug, StructOpt)]
enum Opt {
    /// Download Intel-MKL library
    Download {
        /// Install destination. Default is `$XDG_DATA_HOME/intel-mkl-tool`
        path: Option<PathBuf>,
    },

    /// Seek Intel-MKL library
    ///
    /// 1. pkg-config
    /// 2. `$XDG_DATA_HOME/intel-mkl-tool`
    /// will be sought.
    Seek {},

    /// Package Intel MKL libraries into an archive
    Package { path: PathBuf },
}

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let opt = Opt::from_args();

    match opt {
        Opt::Download { path } => {
            let path = if let Some(path) = path {
                path
            } else {
                xdg_home_path()
            };
            download_default(&path)?;
        }

        Opt::Seek {} => {
            for lib in Entry::available() {
                println!("{}", lib.name());
                for (name, path) in lib.targets().iter() {
                    println!("  {:<25} at {}", name, path.as_ref().unwrap().display());
                }
            }
        }

        Opt::Package { path } => {
            let _out = package(&path)?;
        }
    }
    Ok(())
}
