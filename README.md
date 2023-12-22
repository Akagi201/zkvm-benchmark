# zkvm-benchmark

Benchmarks between zkWasm and zkVM

## Test steps

```sh
cd zkvm/wasm
just build
just dry-run
```

```sh
cd zkwasm/fib
just build
just run-wasi
just dry-run
```

## Benchmark

Install [hyperfine](https://github.com/sharkdp/hyperfine) first.

```sh
brew install hyperfine
```
