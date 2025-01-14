use coremark::engines::{wasmtime_coremark, wasmedge_coremark, wasmer_coremark, wasm3_coremark, wasmi_coremark, wamr_coremark};

fn main() {
    // let coremark_wasm = include_bytes!("../../../target/wasm32-unknown-unknown/release/coremark_wasm.wasm");
    let coremark_wasm = include_bytes!("../coremark-minimal.wasm");

    println!("Running Coremark tests... [should take 12..20 seconds per engine]");

    let engines = ["wasmtime", "wasmedge", "wasmer", "wasm3", "wasmi", "wamr"];
    let mut results = vec![];

    for &engine in &engines {
        let result = match engine {
            "wasmtime" => wasmtime_coremark(coremark_wasm),
            "wasmedge" => wasmedge_coremark(coremark_wasm),
            "wasmer" => wasmer_coremark(coremark_wasm),
            "wasm3" => wasm3_coremark(coremark_wasm),
            "wasmi" => wasmi_coremark(coremark_wasm),
            "wamr" => wamr_coremark(coremark_wasm),
            _ => unreachable!(),
        };

        match result {
            Ok(value) => results.push((engine, value)),
            Err(e) => {
                eprintln!("Error occurred: {}", e);
            }
        }
    }

    println!("\nResults:\n");
    println!("| Engine   | Result   |\n|----------|----------|");
    for (engine, result) in results {
        println!("| {:<8} | {:<8.2} |", engine, result);
    }
}
