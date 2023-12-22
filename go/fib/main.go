package main

import "fmt"

// fibonacci
func main() {
	var a0, a1 int64
	a0 = 0
	a1 = 1
	for i := 2; i <= 100; i++ {
		a0, a1 = a1, (a0+a1) % (1<<32)
	}
	fmt.Println(a1)
}