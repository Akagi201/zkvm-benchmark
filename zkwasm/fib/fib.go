package main

//go:wasmimport env require
//go:noescape
func require(uint32)

func main() {
	var a0, a1 int64
	a0 = 0
	a1 = 1
	for i := 2; i <= 100; i++ {
		a0, a1 = a1, (a0+a1) % (1<<32)
	}
	var an int64 = 3314859971
	if an != a1 {
		require(0)
	}
}
