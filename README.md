# zkvm-benchmark

Benchmarks between zkWasm and zkVM

## Test steps

```sh
cd zkvm/wasm
just build
just wat
just dry-run
```

```sh
cd zkwasm/fib
just build
just run-wasi
just dry-run
```
