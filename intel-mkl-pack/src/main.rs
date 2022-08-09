mod pack;

use anyhow::Result;
use colored::Colorize;
use intel_mkl_tool::{Config, Library};
use ocipkg::ImageName;
use pack::pack;
use std::time::Instant;

fn main() -> Result<()> {
    for cfg in Config::possibles() {
        let lib = Library::new(cfg)?;
        let (year, _, update) = lib.version()?;
        let registry = "ghcr.io/rust-math/intel-mkl-src";
        let name = ImageName::parse(&format!("{}/{}:{}.{}", registry, cfg, year, update))?;
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
