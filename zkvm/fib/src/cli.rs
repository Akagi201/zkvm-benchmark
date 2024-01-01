use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[clap(long, short)]
    pub dry_run: bool,
    #[clap(long, short)]
    pub number: u64,
}
