use anyhow::{bail, Result};
use intel_mkl_tool::*;
use std::{env, path::PathBuf, str::FromStr};
use structopt::StructOpt;

/// CLI tool for intel-mkl crate
#[derive(Debug, StructOpt)]
enum Opt {
    /// Download Intel-MKL library
    Download {
        /// Archive name, e.g. "mkl-static-lp64-iomp". Download all archives if None
        #[structopt(long = "name")]
        name: Option<String>,
        /// Install destination
        #[structopt(short = "o", long = "path")]
        path: PathBuf,
    },

    /// Seek Intel-MKL library
    Seek {},

    /// Package Intel MKL libraries into an archive
    Package {
        #[structopt(long = "name")]
        name: Option<String>,
        #[structopt(short = "o", long = "path")]
        path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Download { name, path } => {
            if let Some(name) = name {
                let cfg = Config::from_str(&name)?;
                cfg.download(&path.join(cfg.to_string()))?;
            } else {
                for cfg in Config::possibles() {
                    println!("Download archive {:<22} into {}", cfg, path.display());
                    cfg.download(&path.join(cfg.to_string()))?;
                }
            }
        }

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

        Opt::Package { name, path } => {
            let path = path.unwrap_or(env::current_dir().unwrap());
            if let Some(name) = name {
                let cfg = Config::from_str(&name)?;
                let entry = Entry::from_config(cfg)?;
                let path = if let Ok(version) = entry.version() {
                    path.join(format!("{}.{}", version.0, version.1))
                } else {
                    path
                };
                let package = entry.package(&path)?;
                println!("Pacakge created: {}", package.display());
            } else {
                for entry in Entry::available() {
                    let path = if let Ok(version) = entry.version() {
                        path.join(format!("{}.{}", version.0, version.1))
                    } else {
                        path.clone()
                    };
                    let package = entry.package(&path)?;
                    println!("Pacakge created: {}", package.display());
                }
            }
        }
    }
    Ok(())
}
