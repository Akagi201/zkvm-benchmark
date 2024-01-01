#[doc = include_str!("../README.md")]
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv};
use wasm_methods::{WASM_INTERP_ELF, WASM_INTERP_ID};

pub fn run_guest(dry_run: bool, wasm: Vec<u8>, pub_values: Vec<u64>) {
    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .write(&pub_values)
        .unwrap()
        .build()
        .unwrap();

    if dry_run {
        let executor = default_executor();
        let _session_info = executor.execute_elf(env, WASM_INTERP_ELF).unwrap();
    } else {
        let prover = default_prover();
        let receipt = prover.prove_elf(env, WASM_INTERP_ELF).unwrap();
        receipt.verify(WASM_INTERP_ID).expect(
            "Code you have proven should successfully verify; did you specify the correct image ID?",
        );
        println!("Proof verified successfully");
    }
}
