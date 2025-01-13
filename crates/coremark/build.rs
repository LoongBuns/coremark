use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let current_project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(&current_project_dir).join("../..");

    install_wasmedge();
    build_benchmark(&workspace_dir);

    println!(
        "cargo:rerun-if-changed={}",
        workspace_dir.join("coremark_wasm/src").to_string_lossy()
    );
}

fn install_wasmedge() {
    let home_dir = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let wasmedge_path = PathBuf::from(&home_dir).join(".wasmedge");

    if !wasmedge_path.exists() {
        let install_status = Command::new("sh")
            .arg("-c")
            .arg("curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash")
            .status();

        if !install_status.map(|s| s.success()).unwrap_or(false) {
            panic!("Failed to install WasmEdge.");
        }
    }
}

fn build_benchmark(workspace_dir: &PathBuf) {
    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--package",
            "coremark_wasm",
        ])
        .current_dir(&workspace_dir)
        .status();

    if !status.map(|s| s.success()).unwrap_or(false) {
        panic!("Failed to build coremark_wasm.");
    }
}
