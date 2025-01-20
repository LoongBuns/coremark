use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use wasmedge_sdk::{
    error::CoreError, params, AsInstance, CallingFrame, ImportObjectBuilder, Instance, Module,
    Store, Vm, WasmValue,
};

fn clock_ms(
    _: &mut (),
    _inst: &mut Instance,
    _frame: &mut CallingFrame,
    _input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_millis() as i64;

    Ok(vec![WasmValue::from_i64(now)])
}

pub fn wasmedge_container(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let mut import_builder = ImportObjectBuilder::new("env", ())?;
    import_builder.with_func::<(), i64>("clock_ms", clock_ms)?;
    let mut import_object = import_builder.build();

    let mut instances = HashMap::new();
    instances.insert(import_object.name().unwrap(), &mut import_object);

    let module = Module::from_bytes(None, &b[..])?;

    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    vm.register_module(None, module)?;
    let result = vm.run_func(None, "run", params!())?;

    Ok(result[0].to_f32())
}
