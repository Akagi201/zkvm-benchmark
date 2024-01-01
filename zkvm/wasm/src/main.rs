use std::{fs::File, io::Read};

use clap::Parser;
mod cli;
use cli::Cli;
mod executor;
use crate::executor::run_guest;

fn main() {
    let cli = Cli::parse();
    let file_path = cli.wasm;
    let mut file = File::open(file_path).unwrap();
    let file_size = file.metadata().unwrap().len() as usize;
    let mut wasm = Vec::with_capacity(file_size);
    file.read_to_end(&mut wasm).unwrap();
    println!("dry_run: {}, public: {:?}", cli.dry_run, cli.public);
    run_guest(cli.dry_run, wasm, cli.public);
}
