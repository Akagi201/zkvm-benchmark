use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[clap(long, short, value_name = "WASM")]
    pub wasm: String,
}
