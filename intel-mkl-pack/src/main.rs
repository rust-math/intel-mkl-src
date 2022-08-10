mod pack;

const REGISTRY: &str = "ghcr.io/rust-math/intel-mkl-src";

use anyhow::Result;
use colored::Colorize;
use intel_mkl_tool::{Config, Library};
use ocipkg::ImageName;
use pack::pack;
use std::time::Instant;

fn main() -> Result<()> {
    let run_id: u64 = std::env::var("GITHUB_RUN_ID")
        .unwrap_or("0".to_string()) // fallback value for local testing
        .parse()?;
    for cfg in Config::possibles() {
        let lib = Library::new(cfg)?;
        let (year, _, update) = lib.version()?;
        let name = ImageName::parse(&format!(
            "{}/{}:{}.{}-{}",
            REGISTRY, cfg, year, update, run_id
        ))?;
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
