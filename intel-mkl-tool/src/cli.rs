use anyhow::*;
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
                intel_mkl_tool::home_library_path()
            };
            intel_mkl_tool::download(&path)?;
        }

        Opt::Seek {} => {
            if let Some(path) = intel_mkl_tool::seek_pkg_config() {
                println!("{}", path.display());
                return Ok(());
            }
            if let Some(path) = intel_mkl_tool::seek_home() {
                println!("{}", path.display());
                return Ok(());
            }
            bail!("Intel-MKL not found.");
        }

        Opt::Package { path } => {
            let _out = intel_mkl_tool::package(&path)?;
        }
    }
    Ok(())
}
