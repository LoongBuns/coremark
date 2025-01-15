# coremark

A latest coremark benchmark for wasm runtime.

**Source**: https://github.com/eembc/coremark

**Inspire:** https://github.com/patractlabs/wasm-coremark-rs

## Start benchmark

You can start benchamrk by command:

```sh
cargo run -p coremark
```

**Interpreter result:**

* Test wasm: [coremark-minimal.wasm](https://github.com/wasm3/wasm-coremark/blob/main/coremark-minimal.wasm)
* Test environment: github codespace

| Engine   | Result   |
|----------|----------|
| wasmtime | 10013.52 |
| wasmedge | 96.46    |
| wasmer   | 13374.35 |
| wasm3    | 243.92   |
| wasmi    | 48.34    |

* Test wasm: coremark_wasm.wasm (include in project)
* Test environment: github codespace

| Engine     | Result(ms)         |
|------------|--------------------|
| wasmtime   | 120.00             |
| wasmedge   | 17154.00           |
| wasmer     | 129.00             |
| wasm3      | 15210.00           |
| wasmi      | 52189.00           |