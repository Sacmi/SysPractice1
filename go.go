package main

func factorial(n uint64) uint64 {
	if n == 0 {
		return 1
	}
	return n * factorial(n-1)
}

func main() {
	for i := 0; i < 1000000; i++ {
		factorial(20)
	}
}