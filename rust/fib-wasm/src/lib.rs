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
        if expected != result {
            require(0);
        }
    }
}

pub fn fib(n: u64) -> u64 {
    let mut a0: u64 = 0;
    let mut a1: u64 = 1;
    for _ in 2..=n {
        (a0, a1) = (a1, (a0 + a1) % (1 << 32));
    }
    a0
}
