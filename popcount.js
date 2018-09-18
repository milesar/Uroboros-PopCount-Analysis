
const m1 = 0x55555555
const m2 = 0x33333333
const m4 = 0x0f0f0f0f
const h01 = 0x01010101

const {
  performance,
  PerformanceObserver
} = require('perf_hooks');

function popcount_1_control(x) {
  var c = 0;
  for (var i = 0; i < 32; i++) {
    if (x & 1)
      c++;
    x >>>= 1;
  }
  return c;
}

function popcount_kernighan(n) {
  n >>> 0;
  var count = 0;
  while (n) {
    count += n & 0x1;
    n >>>= 1;
  }
  return count;
}

function popcount64c(x) {
  x >>> 0;
	x -= (x >>> 1) & m1;
	x = (x & m2) + ((x >>> 2) & m2);
	x = (x + (x >>> 4)) & m4;
	return (x * h01) >>> 56;
}

function test(popcount, name) {
	const N = 10000;
	var nums = new Uint32Array(N);
	nums[0] = 0;
	nums[1] = 0xffffffff;

	for (var i = 2; i < N; i++) {
		nums[i] = Math.random() >>> 0
	}

	var last_sum = 0;
	var sum = 0;
	var start = performance.now();
	for (var i = 0; i < N; i++) {
		sum += popcount(nums[i])
	}
  var end = performance.now();
	var elapsed = end - start;
	var speed = elapsed / N * 1000000;
	if (sum == last_sum) {
		console.log("checkrep error: " + sum + " != " + last_sum);
	}
	last_sum = sum
	console.log(name + "\t" + speed + " ns / op\n")
}

test(popcount_1_control, "popcount_1_control");
test(popcount_kernighan, "popcount_kernighan");
test(popcount64c, "popcount64c");
