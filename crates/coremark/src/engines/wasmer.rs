use core::error::Error;
use core::result::Result;

use wasmer::{
    imports, Function, FunctionType, Instance, Module, Store, Type, TypedFunction, Value,
};

use super::clock_ms;

pub fn wasmer_coremark(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let mut store = Store::default();
    let module = Module::new(&store, &b[..])?;

    let clock_ms_host_signature = FunctionType::new(vec![], vec![Type::I32]);
    let clock_ms_host = Function::new(&mut store, &clock_ms_host_signature, |_| {
        Ok(vec![Value::I32(clock_ms() as i32)])
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
