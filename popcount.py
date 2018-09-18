import time
import random

m1 = 0x5555555555555555
m2 = 0x3333333333333333
m4 = 0x0f0f0f0f0f0f0f0f
h01 = 0x0101010101010101


def popcount_1_control(x):

    c = 0
    for i in range(0, 64, 1):
        if x & 1:
            c += 1
        x >>= 1

    return c


def popcount_kernighan(n):

    count = 0
    while n:
        n &= n - 1
        count += 1
    return count


def popcount64c(x):
    x -= (x >> 1) & m1
    x = (x & m2) + ((x >> 2) & m2)
    x = (x + (x >> 4)) & m4
    return (x * h01) >> 56


def test(popcount, name):
    N = 10000
    nums = [0] * N
    nums[0] = 0x0
    nums[1] = 0xffffffffffffffff

    for i in range(2, N, 1):
        nums[i] = random.getrandbits(64)

    last_sum = 0
    sum = 0
    start = time.perf_counter()
    for i in range(0, N, 1):
        sum += popcount(nums[i])
    end = time.perf_counter()
    speed = (end - start) * 1E9 / N   # convert output to nanoseconds
    if sum is not last_sum and last_sum is not 0:
        print("checkrep error: " + str(sum) + " != " + str(last_sum) + "\n")

    # basic testing against control method.
    # for i in range(0, N, 1):
    #    testcount = popcount(nums[i])
    #    control = popcount_1_control(nums[i])
    #    if testcount is not control:
    #        print("incorrect results for" + str(testcount) + " != " + str(control))

    last_sum = sum
    print(name + "\t" + str(speed) + "ns / op\n")


if __name__ == '__main__':
    # control runs first, to set and help advance the correct sum
    test(popcount_1_control, "popcount_1_control")
    test(popcount_kernighan, "popcount_kernighan")
    test(popcount64c, "popcount64c")
