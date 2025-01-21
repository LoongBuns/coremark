use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(&manifest_dir);

    let rustflags = "-C link-arg=--initial-memory=65536 \
                     -C link-arg=-zstack-size=8192 \
                     -C link-arg=--export=__heap_base \
                     -C link-arg=--export=__data_end \
                     -C link-arg=--strip-all";

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--package",
            "coremark_wasm",
        ])
        .env("RUSTFLAGS", rustflags)
        .current_dir(&workspace_dir)
        .status();

    if !status.map(|s| s.success()).unwrap_or(false) {
        panic!("Failed to build coremark_wasm.");
    }

    println!(
        "cargo:rerun-if-changed={}",
        workspace_dir.join("crates/coremark_wasm/src").to_string_lossy()
    );
}
