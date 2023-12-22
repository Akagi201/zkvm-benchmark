use std::{fs::File, io::Read};

use clap::Parser;
use risc0_zkvm::{default_prover, ExecutorEnv};
use wasm_methods::{WASM_INTERP_ELF, WASM_INTERP_ID};

mod cli;
use cli::Cli;

#[allow(dead_code)]
fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}

fn run_guest() {
    let cli = Cli::parse();
    let file_path = cli.wasm;
    let mut file = File::open(file_path).unwrap();
    let file_size = file.metadata().unwrap().len() as usize;
    let mut wasm = Vec::with_capacity(file_size);
    file.read_to_end(&mut wasm).unwrap();

    // let file_path = "./fib.wat";
    // let mut wat = String::new();
    // file.read_to_string(&mut wat).unwrap();

    // let wasm = wat2wasm(&wat).expect("Failed to parse_str");

    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, WASM_INTERP_ELF).unwrap();

    receipt.verify(WASM_INTERP_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
    // let result: i32 = receipt.journal.decode().unwrap();

    // result
}

fn main() {
    run_guest();
    // println!("In host, fib {} - {}", 100, res);
}
