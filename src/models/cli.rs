use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub todo: String,
}