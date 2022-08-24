use intel_mkl_tool::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    let mut num_not_found = 0;
    for cfg in Config::possibles() {
        log::info!("Seek {}", cfg);
        if let Ok(lib) = Library::new(cfg) {
            log::info!("{:?}", lib);
        } else {
            num_not_found += 1;
        }
    }
    ExitCode::from(num_not_found)
}
