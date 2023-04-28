use clap::Parser;
use todo::models::cli::Cli;
use todo::models::db::Db;
use todo::models::env::Env;
use todo::models::todo::Todo;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let db = Db::new(Env::Prod).await?;

    let now = chrono::offset::Utc::now().timestamp_millis();

    let todo = Todo::new(
        Uuid::new_v4().to_string(), // id
        args.todo, //  title
        false, // is_completed
        now, // created_at
        now // updated_at
    );

    db.insert(&todo).await?;

    Ok(())
}
