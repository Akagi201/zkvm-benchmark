#[doc = include_str!("../README.md")]
use fib_methods::{FIB_CALC_ELF, FIB_CALC_ID};
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv};

pub fn run_guest(dry_run: bool, n: u64) {
    let env = ExecutorEnv::builder().write(&n).unwrap().build().unwrap();

    if dry_run {
        let executor = default_executor();
        let session_info = executor.execute_elf(env, FIB_CALC_ELF).unwrap();
        let result: u64 = session_info.journal.decode().unwrap();
        println!("fib({n}) = {result}");
    } else {
        let prover = default_prover();
        let receipt = prover.prove_elf(env, FIB_CALC_ELF).unwrap();
        let result: u64 = receipt.journal.decode().unwrap();
        println!("fib({n}) = {result}");
        receipt.verify(FIB_CALC_ID).expect(
            "Code you have proven should successfully verify; did you specify the correct image ID?",
        );
        println!("Proof verified successfully");
    }
}
