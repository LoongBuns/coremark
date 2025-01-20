use std::error::Error;
use std::ffi::c_void;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::RuntimeBuilder, value::WasmValue,
};

extern "C" fn clock_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_millis() as i64
}

pub fn wamr_container(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let runtime = RuntimeBuilder::new("env")
        .use_system_allocator()
        .run_as_interpreter()
        .register_host_function("clock_ms", clock_ms as *mut c_void)
        .build()?;

    let module = Module::from_vec(&runtime, Vec::from(&b[..]), "")?;

    let instance = Instance::new(&runtime, &module, 2 * 1024)?;

    let function = Function::find_export_func(&instance, "run")?;

    if let WasmValue::F32(res) = function.call(&instance, &vec![])? {
        Ok(res)
    } else {
        panic!("Failed running coremark in wamr");
    }
}
