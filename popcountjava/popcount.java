package popcountjava;

public class Popcount {
int popcount_1_control(long x) {
        int c = 0;
        for (int i = 0; i < 64; i++) {
                if (x & 1)
                        c++;
                x >>= 1;
        }
        return c;
}

int popcount_kernighan(long n) {
        int count = 0;
        while (n) {
                count += n & 0x1;
                n >>= 1;
        }
        return count;
}

int popcount64c(long x) {
        x -= (x >> 1) & m1;
        x = (x & m2) + ((x >> 2) & m2);
        x = (x + (x >> 4)) & m4;
        return (x * h01) >> 56;
}

void test(popcount, String name) {
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

public static void main(String[] args) {
        test(popcount_1_control, "popcount_1_control");
        test(popcount_kernighan, "popcount_kernighan");
        test(popcount64c, "popcount64c");

        System.exit(0);
}
}
