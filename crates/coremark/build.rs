use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let current_project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(&current_project_dir).join("../..");

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--package",
            "coremark_benchmark",
        ])
        .current_dir(&workspace_dir)
        .status();

    if !status.map(|s| s.success()).unwrap_or(false) {
        panic!("Failed to build coremark_benchmark.");
    }

    println!(
        "cargo:rerun-if-changed={}",
        workspace_dir.join("coremark_benchmark/src").to_string_lossy()
    );
}
