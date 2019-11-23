use failure::*;
use std::{env, path::PathBuf};
use structopt::StructOpt;

/// CLI tool for intel-mkl crate
#[derive(Debug, StructOpt)]
enum Opt {
    Download { path: Option<PathBuf> },
    Seek {},
}

fn main() -> Fallible<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let opt = Opt::from_args();

    match opt {
        Opt::Download { path } => {
            let path = if let Some(path) = path {
                path
            } else {
                let data_dir = dirs::data_local_dir().unwrap();
                data_dir.join("intel-mkl-tool")
            };
            intel_mkl_tool::download(&path)?;
        }
        Opt::Seek {} => {
            let paths = intel_mkl_tool::seek_pkg_config()?;
            println!("{:?}", paths);
        }
    }
    Ok(())
}
