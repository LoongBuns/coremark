use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
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
