use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[clap(long, short)]
    pub dry_run: bool,
    #[clap(long, short, value_delimiter = ',')]
    pub public: Vec<u64>,
    #[clap(long, short, value_name = "WASM")]
    pub wasm: String,
}
