package main

import "math/big"

//go:wasmimport env wasm_input
//go:noescape
func wasm_input(isPublic uint32) uint64

//go:wasmimport env require
//go:noescape
func require(uint32)

func fib_iter(n *big.Int) *big.Int {
	var a0, a1, a2, i big.Int
	a0.SetInt64(0)
	a1.SetInt64(1)
	for i.SetInt64(2); i.Cmp(n) <= 0; i.Add(&i, big.NewInt(1)) {
		a2.Add(&a0, &a1)
		// a0, a1 = a1, a2, this is wrong of big.Int
		a0.Set(&a1)
		a1.Set(&a2)

	}
	return &a1
}

// fibonacci
func main() {
	num := wasm_input(1)
	var n big.Int
	n.SetUint64(num)
	result := fib_iter(&n)
	expected := wasm_input(0)
	if result.Uint64() != expected {
		require(1)
	}
}
