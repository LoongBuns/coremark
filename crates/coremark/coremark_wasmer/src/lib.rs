use std::error::Error;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use wasmer::{
    imports, Function, FunctionType, Instance, Module, Store, Type, TypedFunction, Value,
};

pub fn wasmer_container(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let mut store = Store::default();
    let module = Module::new(&store, &b[..])?;

    let clock_ms_host_signature = FunctionType::new(vec![], vec![Type::I64]);
    let clock_ms_host = Function::new(&mut store, &clock_ms_host_signature, |_| {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis() as i64;

        Ok(vec![Value::I64(now)])
    });

    let import_object = imports! {
        "env" => {
            "clock_ms" => clock_ms_host,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let run: TypedFunction<(), f32> = instance.exports.get_function("run")?.typed(&mut store)?;

    Ok(run.call(&mut store)?)
}
