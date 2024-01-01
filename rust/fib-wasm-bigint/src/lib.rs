use num_bigint::BigUint;
use num_traits::{One, Zero};
use wasm_bindgen::prelude::*;

extern "C" {
    pub fn wasm_input(is_public: u32) -> u64;
    pub fn require(cond: u32);
}

#[wasm_bindgen]
pub fn zkmain() {
    let input = unsafe { wasm_input(1) };
    // let input = 100;
    let result = fib(input);
    unsafe {
        let expected = wasm_input(0);
        if expected != result[0] {
            require(0);
        }
    }
}

pub fn fib(n: u64) -> Vec<u64> {
    let mut a0: BigUint = Zero::zero();
    let mut a1: BigUint = One::one();
    for _ in 2..=n {
        (a0, a1) = (a1.clone(), (a0 + a1));
    }
    let mod_result = a0 % BigUint::from(1u64 << 32);
    mod_result.to_u64_digits()
}
