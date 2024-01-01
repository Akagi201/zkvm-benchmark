mod cli;
mod executor;

use clap::Parser;

use crate::{cli::Cli, executor::run_guest};

fn main() {
    let cli = Cli::parse();
    run_guest(cli.dry_run, cli.number);
}
