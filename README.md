# zkvm-benchmark

Benchmarks between zkWasm and zkVM

## Dependencies

Before running, make sure your have installed the following tools:

* [just](https://github.com/casey/just): a command runner
* [hyperfine](https://github.com/sharkdp/hyperfine): a benchmarking tool
* wasm-opt: from [binaryen](https://github.com/WebAssembly/binaryen), for wasm to wasm optimization.
* delphinus-cli: from [zkWasm](https://github.com/DelphinusLab/zkWasm), to run zkWasm program.
* lgo: an alias of the `go` binary built from [zkgo](https://github.com/ethstorage/go/tree/zkGo), for build go program to wasm.

## Benchmark

Run Benchmarks with hyperfine.

```sh
just clean # optional
just build
just dry-run
```

Tested on my Mac Studio M1 Max

![zkvm_bench](assets/zkvm_bench.png)

## Test manually

Build rust version fibonacci wasm program, and copy to zkVM folder.

```sh
cd rust/fib-wasm
just build
just opt
just copy
```

dry-run Rust and Go version fibonacci wasm program on zkVM.

```sh
cd zkvm/wasm
just build
just opt
just dry-run-rust
just dry-run-go
```

dry-run Go version fibonacci wasm program on zkWasm.

```sh
cd zkwasm/fib
just build
just run-wasi
just dry-run
```
