use std::error::Error;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use wasmtime::*;

pub fn wasmtime_container(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let engine = Engine::default();

    let module = Module::new(&engine, &b[..])?;

    let mut store = Store::<u32>::new(&engine, 64);

    let mut linker = Linker::new(&engine);
    linker.func_wrap("env", "clock_ms", || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis() as i64
    })?;

    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), f32>(&mut store, "run")?;

    Ok(run.call(&mut store, ())?)
}
