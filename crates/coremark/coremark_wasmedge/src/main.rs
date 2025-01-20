use coremark_wasmedge::wasmedge_container;

fn main() {
    let coremark_wasm = include_bytes!("../../../target/wasm32-unknown-unknown/release/coremark_wasm.wasm");

    println!("Running Coremark test on wasmedge");

    match wasmedge_container(coremark_wasm) {
        Ok(value) => {
            println!("wasmedge: {}", value);
        },
        Err(e) => {
            eprintln!("Error occurred: {}", e);
        }
    }
}
