use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub todo: Option<String>,
    #[clap(long, short)]
    pub complete: Option<String>,
    #[clap(long, short, action)]
    pub list_todos: bool
}