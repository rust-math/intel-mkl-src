use anyhow::*;
use intel_mkl_tool::*;
use log::*;
use std::{env, path::PathBuf};
use structopt::StructOpt;

/// CLI tool for intel-mkl crate
#[derive(Debug, StructOpt)]
enum Opt {
    /// Download Intel-MKL library
    Download {
        /// Archive name, e.g. "mkl-static-lp64-iomp". Download all archives if None
        name: Option<String>,
        /// Install destination. Default is `$XDG_DATA_HOME/intel-mkl-tool/${MKL_VERSION}/`
        path: Option<PathBuf>,
    },

    /// Seek Intel-MKL library
    ///
    /// 1. pkg-config
    /// 2. `$XDG_DATA_HOME/intel-mkl-tool`
    /// will be sought.
    Seek {},

    /// Package Intel MKL libraries into an archive
    Package {
        name: Option<String>,
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let opt = Opt::from_args();

    match opt {
        Opt::Download { name, path } => {
            let path = path.unwrap_or(xdg_home_path());
            if let Some(name) = name {
                let cfg = Config::from_str(&name)?;
                cfg.download(&path)?;
            } else {
                for cfg in Config::possible() {
                    info!(
                        "Download archive {:<22} into {}",
                        cfg.name(),
                        path.display()
                    );
                    cfg.download(&path)?;
                }
            }
        }

        Opt::Seek {} => {
            for lib in Entry::available() {
                if let Ok(version) = lib.version() {
                    println!("{:<22}: {}.{}", lib.name(), version.0, version.1);
                } else {
                    println!("{:<22}", lib.name());
                }
                for (path, name) in &lib.files() {
                    println!("  {:<25} at {}", name, path.display());
                }
            }
        }

        Opt::Package { name, out } => {
            let out = out.unwrap_or(env::current_dir().unwrap());
            if let Some(name) = name {
                let cfg = Config::from_str(&name)?;
                let entry = Entry::from_config(cfg)?;
                let out = if let Ok(version) = entry.version() {
                    out.join(format!("{}.{}", version.0, version.1))
                } else {
                    out
                };
                let package = entry.package(&out)?;
                info!("Pacakge created: {}", package.display());
            } else {
                for entry in Entry::available() {
                    let out = if let Ok(version) = entry.version() {
                        out.join(format!("{}.{}", version.0, version.1))
                    } else {
                        out.clone()
                    };
                    let package = entry.package(&out)?;
                    info!("Pacakge created: {}", package.display());
                }
            }
        }
    }
    Ok(())
}
