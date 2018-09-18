package main

import (
	"fmt"
	"math/rand"
	"time"
)

type fn func(uint64) uint64

func popcount_1_control(x uint64) uint64 {
	var c uint64 = 0
	for i := 0; i < 64; i++ {
		if x&1 > 0 {
			c++
		}
		x >>= 1
	}
	return c
}

func popcount_kernighan(n uint64) uint64 {
	var c uint64 = 0
	for n > 0 {
		c += n & 0x1
		n >>= 1
	}
	return c
}

const m1 = 0x5555555555555555
const m2 = 0x3333333333333333
const m4 = 0x0f0f0f0f0f0f0f0f
const h01 = 0x0101010101010101

func popcount64c(x uint64) uint64 {
	x -= (x >> 1) & m1
	x = (x & m2) + ((x >> 2) & m2)
	x = (x + (x >> 4)) & m4
	return (x * h01) >> 56
}

func test(popcount fn, name string) {
	const N = 10000
	nums := [N]uint64{0}
	nums[0] = 0
	nums[1] = 0xffffffffffffffff
	rand.Seed(time.Now().UnixNano())

	for i := 2; i < N; i++ {
		nums[i] = rand.Uint64()
	}

	var last_sum uint64 = 0
	var sum uint64 = 0
	start := time.Now()
	for i := 0; i < N; i++ {
		sum += popcount(nums[i])
	}
	elapsed := time.Since(start)
	speed := elapsed / N
	if sum == last_sum {
		fmt.Printf("checkrep error: %d = %d", sum, last_sum)
	}

	// error checking against control implementation
	for i := 0; i < N; i++ {
		if popcount(nums[i]) != popcount_1_control(nums[i]) {
			fmt.Printf("incorrect count")
			break
		}
	}
	last_sum = sum
	fmt.Printf("%v\t %d ns / op\n", name, speed)
}

func main() {
	test(popcount_1_control, "popcount_1_control")
	test(popcount_kernighan, "popcount_kernighan")
	test(popcount64c, "popcount64c")
}
