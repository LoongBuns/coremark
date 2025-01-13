use core::error::Error;
use core::result::Result;
use std::collections::HashMap;

use wasmedge_sdk::{
    error::CoreError, params, AsInstance, CallingFrame, ImportObjectBuilder, Instance, Module,
    Store, Vm, WasmValue,
};

use super::clock_ms;

fn clock_ms_host(
    _: &mut (),
    _inst: &mut Instance,
    _frame: &mut CallingFrame,
    _input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    Ok(vec![WasmValue::from_i64(clock_ms())])
}

pub fn wasmedge_coremark(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let mut import_builder = ImportObjectBuilder::new("env", ())?;
    import_builder.with_func::<(), i64>("clock_ms", clock_ms_host)?;
    let mut import_object = import_builder.build();

    let mut instances = HashMap::new();
    instances.insert(import_object.name().unwrap(), &mut import_object);

    let module = Module::from_bytes(None, &b[..])?;

    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    vm.register_module(None, module)?;
    let result = vm.run_func(None, "run", params!())?;

    Ok(result[0].to_f32())
}
