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
* Test environment: github codespace (2core, 8RAM)

| Engine   | Result   |
|----------|----------|
| wasmtime | 10013.52 |
| wasmedge | 96.46    |
| wasmer   | 13374.35 |
| wasm3    | 243.92   |
| wasmi    | 48.34    |