use clap::Parser;
use todo::models::cli::Cli;
use todo::models::db::Db;
use todo::models::env::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Cli::parse();
    let _db = Db::new(Env::Prod).await?;

    Ok(())
}
