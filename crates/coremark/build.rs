use std::{env, fs};
use std::path::Path;
use std::process::Command;

fn main() {
    let current_project_dir = env::var("CARGO_MANIFEST_DIR").expect("Unable to find workspace directory");
    let workspace_dir = Path::new(&current_project_dir).join("../..");

    let status = Command::new("cargo")
        .args(["build", "--release", "--target", "wasm32-unknown-unknown", "--package", "coremark_wasm"])
        .current_dir(&workspace_dir)
        .status()
        .expect("Failed to build wasm_project");

    if !status.success() {
        panic!("Wasm build failed");
    }

    let wasm_file = workspace_dir.join("target/wasm32-unknown-unknown/release/coremark_wasm.wasm");
    let dest_file = Path::new(&current_project_dir).join("src/coremark_wasm.wasm");

    fs::copy(&wasm_file, &dest_file).expect("Failed to copy wasm file");

    println!(
        "cargo:rerun-if-changed={}",
        workspace_dir.join("coremark_wasm/src").to_string_lossy()
    );
}
