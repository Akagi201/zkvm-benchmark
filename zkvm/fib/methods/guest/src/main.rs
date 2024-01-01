#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let n: u64 = env::read();
    let result = fib(n);
    env::commit(&result);
}

pub fn fib(n: u64) -> u64 {
    let mut a0: u64 = 0;
    let mut a1: u64 = 1;
    for _ in 2..=n {
        (a0, a1) = (a1, (a0 + a1) % (1 << 32));
    }
    a0
}