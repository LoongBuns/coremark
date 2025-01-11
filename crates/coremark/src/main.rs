use coremark::engines::{wasmtime_coremark, wasm3_coremark, wasmi_coremark, wamr_coremark};

fn main() {
    let coremark_wasm = include_bytes!("./coremark_wasm.wasm");

    println!("Running Coremark tests... [should take 12..20 seconds per engine]");

    let engines = ["wasmtime", "wasm3", "wasmi", "wamr"];
    let mut results = vec![];

    for &engine in &engines {
        let result = match engine {
            "wasmtime" => wasmtime_coremark(coremark_wasm),
            "wasm3" => wasm3_coremark(coremark_wasm),
            "wasmi" => wasmi_coremark(coremark_wasm),
            "wamr" => wamr_coremark(coremark_wasm),
            _ => unreachable!(),
        };
        results.push((engine, result.unwrap()));
    }

    println!("\nResults:\n");
    println!("| Engine   | Result   |\n|----------|----------|");
    for (engine, result) in results {
        println!("| {:<8} | {:<8.2} |", engine, result);
    }
}
