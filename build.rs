
fn main() {
    println!("cargo:rustc-link-lib=static=mkl_intel_ilp64");
    println!("cargo:rustc-link-lib=static=mkl_intel_thread");
    println!("cargo:rustc-link-search=native=./mkl_lib");
}
