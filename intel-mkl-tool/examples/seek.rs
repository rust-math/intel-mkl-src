use intel_mkl_tool::*;

fn main() {
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
            println!(" Not found");
        }
    }
}
