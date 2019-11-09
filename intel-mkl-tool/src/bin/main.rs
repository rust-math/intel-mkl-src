use failure::*;
use std::{env, path::PathBuf};
use structopt::StructOpt;

/// CLI tool for intel-mkl crate
#[derive(Debug, StructOpt)]
enum Opt {
    Download { path: PathBuf },
    PkgConfig {},
}

fn main() -> Fallible<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let opt = Opt::from_args();

    match opt {
        Opt::Download { path } => {
            intel_mkl_tool::download(&path)?;
            println!("{:?}", path);
        }
        Opt::PkgConfig {} => {
            unimplemented!();
        }
    }
    Ok(())
}
