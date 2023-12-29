#[doc = include_str!("../README.md")]
use hello_world_methods::MULTIPLY_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// This is a Hello World demo for the RISC Zero zkVM.
// By running the demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a*b == 391.
// The factors a and b are kept secret.

// Compute the product a*b inside the zkVM
pub fn fib() -> Receipt {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, MULTIPLY_ELF).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    // let c: u64 = receipt.journal.decode().expect(
    //     "Journal output should deserialize into the same types (& order) that it was written",
    // );

    // Report the product
    // println!("I know the factors of {}, and I can prove it!", c);

    receipt
}
