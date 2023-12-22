# build all artifacts
build:
  just -f rust/fib-wasm/Justfile build
  just -f rust/fib-wasm/Justfile opt
  just -f rust/fib-wasm/Justfile copy
  just -f zkvm/wasm/Justfile build
  just -f zkvm/wasm/Justfile opt
  just -f zkwasm/fib/Justfile build
  just -f zkwasm/fib/Justfile opt

# clean all artifacts
clean:
  just -f rust/fib-wasm/Justfile clean
  just -f zkvm/wasm/Justfile clean
  just -f zkwasm/fib/Justfile clean

# dry-run all with hyperfine
dry-run:
  hyperfine -r 1 'just -f zkvm/wasm/Justfile dry-run-rust' 'just -f zkvm/wasm/Justfile dry-run-go' 'just -f zkwasm/fib/Justfile dry-run'
