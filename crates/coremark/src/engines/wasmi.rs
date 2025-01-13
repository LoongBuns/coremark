use std::error::Error;
use std::result::Result;

use wasmi::{Engine, Func, Linker, Module, Store};

use super::clock_ms;

pub fn wasmi_coremark(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let engine = Engine::default();

    let module = Module::new(&engine, &b[..])?;

    let mut store = Store::new(&engine, 64);
    let func = Func::wrap(&mut store, || clock_ms());

    let mut linker = <Linker<i32>>::new(&engine);
    linker.define("env", "clock_ms", func)?;
    let instance = linker
        .instantiate(&mut store, &module)?
        .start(&mut store)?;

    let run = instance.get_typed_func::<(), f32>(&store, "run")?;

    Ok(run.call(&mut store, ())?)
}
