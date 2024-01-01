#![allow(unused)]
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

fn fib_recur(n: BigUint) -> BigUint {
    if n < BigUint::from(2_u32) {
        BigUint::from(n)
    } else {
        fib_recur(n.clone() - 1_u32) + fib_recur(n - 2_u32)
    }
}

fn fib_iter(n: BigUint) -> BigUint {
    let mut a0: BigUint = Zero::zero();
    let mut a1: BigUint = One::one();
    let mut i = BigUint::from(2_u32);
    while i <= n {
        (a0, a1) = (a1.clone(), (a0 + a1));
        i = i + BigUint::from(1_u32);
    }
    a0
}

fn fib_u64_mod(n: u64) -> u64 {
    let mut a0: u64 = 0;
    let mut a1: u64 = 1;
    for _ in 2..=n {
        (a0, a1) = (a1, (a0 + a1) % (1 << 32));
    }
    a0
}

fn main() {
    println!("fib(100) = {}", fib_iter(BigUint::from(100_u32)));
    println!(
        "fib(100)%(1<<32) = {}",
        fib_iter(BigUint::from(100_u32)).to_u128().unwrap() % (1 << 32)
    );
    println!("fib_u64_mod(100) = {}", fib_u64_mod(100_u64));
}
