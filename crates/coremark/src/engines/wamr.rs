use core::error::Error;
use core::ffi::c_void;
use core::result::Result;

use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
};

use super::clock_ms;

extern "C" fn clock_ms_host() -> i64 {
    clock_ms()
}

pub fn wamr_coremark(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .run_as_interpreter()
        .register_host_function("clock_ms", clock_ms_host as *mut c_void)
        .build()?;

    let module = Module::from_vec(&runtime, Vec::from(&b[..]), "env")?;

    let instance = Instance::new(&runtime, &module, 2 * 1024)?;

    let function = Function::find_export_func(&instance, "run")?;

    if let WasmValue::F32(res) = function.call(&instance, &vec![])? {
        Ok(res)
    } else {
        panic!("Failed running coremark in wasmi");
    }
}
