mod wamr;
mod wasm3;
mod wasmedge;
mod wasmer;
mod wasmi;
mod wasmtime;

pub fn clock_ms() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_millis() as i32
}

pub use wamr::wamr_coremark;
pub use wasm3::wasm3_coremark;
pub use wasmedge::wasmedge_coremark;
pub use wasmer::wasmer_coremark;
pub use wasmi::wasmi_coremark;
pub use wasmtime::wasmtime_coremark;
