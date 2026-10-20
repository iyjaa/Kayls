use std::fs;
use std::path::Path;

fn main() {
    let version_path = Path::new("../../VERSION");
    println!("cargo:rerun-if-changed=../../VERSION");
    let version = fs::read_to_string(version_path).unwrap_or_else(|_| "Tsc-v0.0.0".to_string());
    let version = version.trim();
    println!("cargo:rustc-env=TSC_VERSION={}", version);
}
