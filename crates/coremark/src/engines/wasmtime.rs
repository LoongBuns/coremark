use core::error::Error;
use core::result::Result;

use wasmtime::*;

use super::clock_ms;

pub fn wasmtime_coremark(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let engine = Engine::default();

    let module = Module::new(&engine, &b[..])?;

    let mut store = Store::<u32>::new(&engine, 64);

    let mut linker = Linker::new(&engine);
    linker.func_wrap("env", "clock_ms", || clock_ms())?;

    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), f32>(&mut store, "run")?;

    Ok(run.call(&mut store, ())?)
}