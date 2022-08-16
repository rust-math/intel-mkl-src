use intel_mkl_tool::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut num_not_found = 0;
    for cfg in Config::possibles() {
        let lib = Library::new(cfg);
        print!(
            "{:>7} {:>5} {:>4}",
            cfg.link.to_string(),
            cfg.index_size.to_string(),
            cfg.parallel.to_string()
        );
        if let Ok(lib) = lib {
            println!(" {}", lib.library_dir.display());
        } else {
            num_not_found += 1;
            println!(" Not found");
        }
    }
    return ExitCode::from(num_not_found);
}
