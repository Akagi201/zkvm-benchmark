package main

//go:wasmimport env wasm_input
//go:noescape
func wasm_input(isPublic uint32) uint64

//go:wasmimport env require
//go:noescape
func require(uint32)

func fib_u64_mod(n uint64) uint64 {
	var a0, a1, a2 uint64
	a0 = 0
	a1 = 1
	for i := uint64(2); i <= n; i++ {
		a2 = (a0 + a1) % (1 << 32)
		a0, a1 = a1, a2
	}
	return a1
}

// fibonacci
func main() {
	n := wasm_input(1)
	result := fib_u64_mod(n)
	expected := wasm_input(0)
	if result != expected {
		require(1)
	}
}
