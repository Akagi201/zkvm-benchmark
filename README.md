# zkvm-benchmark

Benchmarks between zkWasm and zkVM

## Dependencies

Before running, make sure your have installed the following tools:

* [just](https://github.com/casey/just): a command runner
* [hyperfine](https://github.com/sharkdp/hyperfine): a benchmarking tool
* wasm-opt: from [binaryen](https://github.com/WebAssembly/binaryen), for wasm to wasm optimization.
* [wasm-pack](https://github.com/rustwasm/wasm-pack): Rust → Wasm workflow tool
* delphinus-cli: from [zkWasm](https://github.com/DelphinusLab/zkWasm), to run zkWasm program, built from the latest main branch.
* risczero toolchain: from [risc0](https://github.com/risc0/risc0), to build zkVM program, built from v0.19.1 tag.
* lgo: an alias of the `go` binary built from [zkgo](https://github.com/ethstorage/go/tree/zkGo), for build go program to wasm, built from the latest zkGo branch.

```sh
#just
mkdir -p ~/bin
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin
export PATH="$PATH:$HOME/bin"
# hyperfine
wget https://github.com/sharkdp/hyperfine/releases/download/v1.16.1/hyperfine_1.16.1_amd64.deb
sudo dpkg -i hyperfine_1.16.1_amd64.deb
# wasm-opt
curl -L -O https://github.com/WebAssembly/binaryen/releases/download/version_116/binaryen-version_116-x86_64-linux.tar.gz
tar -xzf binaryen-version_116-x86_64-linux.tar.gz
mv binaryen-version_116-x86_64-linux/bin/wasm-opt ~/bin
# wasm-pack
rustc install stable
rustc default stable 
cargo install wasm-pack
export PATH="$PATH:$HOME/.cargo/bin"
```

## Install risczero toolchain

```sh
git clone https://github.com/risc0/risc0.git
cd risc0
git checkout v0.19.1 # checkout to the latest release
cargo install --path risc0/cargo-risczero
cargo risczero install # installs the latest RISC Zero toolchain
cargo risczero -V
rustup toolchain list --verbose | grep risc0
```

## Install zkGo compiler

```sh
git clone -b zkGo https://github.com/ethstorage/go.git
cd src
./all.bash
# output src/bin
cd ../bin
mv go lgo
export GOROOT="$PATH:your ethstorage go repo path"
# set the bin/ directory to your $PATH env or just copy lgo to your PATH
```

## Install `delphinus-cli`

```sh
git clone https://github.com/DelphinusLab/zkWasm.git
cd zkWasm
cargo build --release
# copy ./target/release/delphinus-cli to one of your $PATH directories
```

## Benchmark

Run Benchmarks with hyperfine.

```sh
just clean # optional
just build
just dry-run
```

Tested on my Mac Studio M1 Max

![macos_profile](assets/macos_profile.jpg)
![zkvm_bench](assets/zkvm_bench.png)

## Test manually

Build rust version fibonacci wasm program, and copy to zkVM/zkWasm folder.

```sh
cd rust/fib-wasm
just build
just opt
just copy
```

dry-run Rust and zkGo version fibonacci wasm program on zkVM.

```sh
cd zkvm/wasm
just build
just opt
just dry-run-rust
just dry-run-go
```

dry-run Rust and zkGo version fibonacci wasm program on zkWasm.

```sh
cd zkwasm/fib
just build
just opt
just dry-run-rust
just dry-run-go
```
