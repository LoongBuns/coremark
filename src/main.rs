use coremark_wamr::wamr_container;
use coremark_wasm3::wasm3_container;
use coremark_wasmedge::wasmedge_container;
use coremark_wasmer::wasmer_container;
use coremark_wasmi::wasmi_container;
use coremark_wasmtime::wasmtime_container;

fn main() {
    let coremark_wasm = include_bytes!("../target/wasm32-unknown-unknown/release/coremark_wasm.wasm");
    // let coremark_wasm = include_bytes!("../coremark-minimal.wasm");

    println!("Running Coremark tests... [should take 12..20 seconds per engine]");

    let engines = ["wasmtime", "wasmedge", "wasmer", "wasm3", "wasmi"];
    let mut results = vec![];

    for &engine in &engines {
        let result = match engine {
            "wasmtime" => wasmtime_container(coremark_wasm),
            "wasmedge" => wasmedge_container(coremark_wasm),
            "wasmer" => wasmer_container(coremark_wasm),
            "wasm3" => wasm3_container(coremark_wasm),
            "wasmi" => wasmi_container(coremark_wasm),
            "wamr" => wamr_container(coremark_wasm),
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
    println!("| Engine     | Result(ms)         |\n|------------|--------------------|");
    for (engine, result) in results {
        println!("| {:<10} | {:<18.2} |", engine, result);
    }
}
