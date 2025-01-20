use std::error::Error;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use wasmi::{Engine, Func, Linker, Module, Store};

pub fn wasmi_container(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let engine = Engine::default();

    let module = Module::new(&engine, &b[..])?;

    let mut store = Store::new(&engine, 64);
    let func = Func::wrap(&mut store, || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis() as i64
    });

    let mut linker = <Linker<i64>>::new(&engine);
    linker.define("env", "clock_ms", func)?;
    let instance = linker
        .instantiate(&mut store, &module)?
        .start(&mut store)?;

    let run = instance.get_typed_func::<(), f32>(&store, "run")?;

    Ok(run.call(&mut store, ())?)
}
