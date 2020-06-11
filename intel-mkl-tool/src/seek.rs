use std::path::*;

pub fn xdg_home_path() -> PathBuf {
    dirs::data_local_dir().unwrap().join("intel-mkl-tool")
}

/// Following pkg-config settings are included in Intel MKL distribution
const PKG_CONFIG_TARGETS: [&str; 8] = [
    "mkl-dynamic-ilp64-iomp",
    "mkl-dynamic-ilp64-seq",
    "mkl-dynamic-lp64-iomp",
    "mkl-dynamic-lp64-seq",
    "mkl-static-ilp64-iomp",
    "mkl-static-ilp64-seq",
    "mkl-static-lp64-iomp",
    "mkl-static-lp64-seq",
];

pub fn seek_pkg_config() -> Vec<(String, pkg_config::Library)> {
    PKG_CONFIG_TARGETS
        .iter()
        .filter_map(|target| {
            let lib = pkg_config::Config::new()
                .cargo_metadata(false)
                .probe(target)
                .ok()?;
            Some((target.to_string(), lib))
        })
        .collect()
}

pub fn seek_xdg_home() -> Vec<(String, PathBuf)> {
    let base = xdg_home_path();
    if !base.exists() {
        return Vec::new();
    }
    base.read_dir()
        .unwrap()
        .flat_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                let name = path.file_name()?.to_str()?.into();
                Some((name, path))
            } else {
                None
            }
        })
        .collect()
}
