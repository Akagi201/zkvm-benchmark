#![no_main]
#![no_std]

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut a0: i64 = 0;
    let mut a1: i64 = 1;
    for _ in 2..=100 {
        (a0, a1) = (a1, (a0 + a1) % (1<<32));
    }
}