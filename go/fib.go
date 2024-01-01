package main

import (
	"fmt"
	"math/big"
)

func fib_recur(n *big.Int) *big.Int {
	if n.Cmp(big.NewInt(2)) < 0 {
		return n
	}
	return new(big.Int).Add(fib_recur(new(big.Int).Sub(n, big.NewInt(1))), fib_recur(new(big.Int).Sub(n, big.NewInt(2))))
}

func fib_iter(n *big.Int) *big.Int {
	var a0, a1, a2, i big.Int
	a0.SetInt64(0)
	a1.SetInt64(1)
	for i.SetInt64(2); i.Cmp(n) <= 0; i.Add(&i, big.NewInt(1)) {
		// a0, a1 = a1, a2, this is wrong for big.Int
		a2.Add(&a0, &a1)
		a0.Set(&a1)
		a1.Set(&a2)
	}
	return &a1
}

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
	var num uint64 = 10
	var n big.Int
	n.SetUint64(num)
	fmt.Println(fib_recur(&n))
	fmt.Println(fib_iter(&n))
	fmt.Println(fib_u64_mod(num))
}
