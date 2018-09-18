func popcount_1_control(x: u64) -> i32 {
	let c = 0
	for x in 0..10 {
		if x&1 > 0 {
			c++
		}
		x >>= 1
	}
	return c
}

func popcount_kernighan(n: u64) -> i32 {
	let c = 0
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

func popcount64c(x: u64) i32 {
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

	let last_sum uint64 = 0
	let sum uint64 = 0
	start := time.Now()
	for i := 0; i < N; i++ {
		sum += popcount(nums[i])
	}
	elapsed := time.Since(start)
	speed := elapsed / N
	if sum == last_sum {
		log.Printf("checkrep error: %d != %d", sum, last_sum)
	}
	last_sum = sum
	log.Printf("%v\t %d ns / op\n", name, speed)
}

func main() {
	test(popcount_1_control, "popcount_1_control")
	test(popcount_kernighan, "popcount_kernighan")
	test(popcount64c, "popcount64c")
}
