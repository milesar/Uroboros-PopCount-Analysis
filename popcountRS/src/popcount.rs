extern crate rand;

use self::rand::Rng;
use std::time::{Duration, Instant};
use std::io::Write;
use std::io::stdout;

pub const MAX: u64 = <u64>::max_value();

fn popcount_1_control(mut x: u64) -> u64 {
    let mut c = 0;
    for i in 0..64 {
        if x & 1 > 0 {
            c += 1;
        }
        x >>= 1;
    }
    return c;
}

fn popcount_kernighan(mut n: u64) -> u64 {
    let mut c = 0;
    while n > 0 {
        c += n & 0x1;
        n >>= 1;
    }
    return c;
}

const m1: u64 = 0x5555555555555555;
const m2: u64 = 0x3333333333333333;
const m4: u64 = 0x0f0f0f0f0f0f0f0f;
const h01: u64 = 0x0101010101010101;

fn popcount64c(mut x: u64) -> u64 {
    x -= (x >> 1) & m1;
    x = (x & m2) + ((x >> 2) & m2);
    x = (x + (x >> 4)) & m4;
    return (x * h01) >> 56;
}

fn test(popcount: fn(u64) -> u64, name: String) {
    let mut nums = [0; 10000];
    nums[0] = 0;
    nums[1] = 0xffffffffffffffff;

    let mut rng = rand::thread_rng();

    for i in 2..10000 {
        nums[i] = rng.gen_range(0, MAX);
    }

    let mut last_sum = 0;
    let mut sum = 0;
    let now = Instant::now();
    for i in 0..10000 {
        sum += popcount(nums[i]);
    }
    let elapsed = now.elapsed().as_secs();
    let speed = elapsed / 10000;
    if sum == last_sum {
        println!("checkrep error: {} != {}\n", sum, last_sum)
    }
    last_sum = sum;
    println!("{}\t {} ns / op\n", name, speed);
    stdout().flush();
}

fn main() {
    let f1 = String::from("popcount_1_control");
    let f2 = String::from("popcount_kernighan");
    let f3 = String::from("popcount64c");
    test(popcount_1_control, f1);
    test(popcount_kernighan, f2);
    test(popcount64c, f3);
}
