use core::error::Error;
use core::result::Result;

use wasm3::{Environment, Module};

use super::clock_ms;

pub fn wasm3_coremark(b: &[u8]) -> Result<f32, Box<dyn Error>> {
    let env = Environment::new()?;
    let rt = env.create_runtime(2 * 1024)?;
    let mut module = rt.load_module(Module::parse(&env, &b[..])?)?;

    module.link_function::<(), i64>("env", "clock_ms", clock_ms_wrap)?;

    Ok(module
        .find_function::<(), f32>("run")?
        .call()?)
}

wasm3::make_func_wrapper!(clock_ms_wrap: clock_ms() -> i64);
