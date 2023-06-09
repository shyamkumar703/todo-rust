use clap::Parser;
use todo::models::cli::Cli;
use todo::models::db::Db;
use todo::models::env::Env;
use todo::models::todo::Todo;
use uuid::Uuid;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let db = Db::new(Env::Prod).await?;

    if let Some(todo_id) = args.complete {
        db.mark_as_complete(&todo_id).await?;
        let todo = db.get(&todo_id).await?;
        let formatted_msg = format!("{} marked as complete", todo.title).green();
        println!("{}", formatted_msg);
        Ok(())
    } else if let Some(todo_name) = args.todo {
        let now = chrono::offset::Utc::now().timestamp_millis();

        let todo = Todo::new(
            Uuid::new_v4().to_string(), // id
            todo_name,                  //  title
            false,                      // is_completed
            now,                        // created_at
            now,                        // updated_at
        );

        db.insert(&todo).await?;

        Ok(())
    } else if let Some(head_limit) = args.top {
        let todo_list = db.get_recent_todos(head_limit).await?;
        display_todo_vec(todo_list);
        Ok(())
    } else if args.list_todos {
        let todo_list = db.list_all().await?;
        display_todo_vec(todo_list);
        Ok(())
    } else if args.incomplete {
        let todo_list = db.get_incomplete_todos().await?;
        display_todo_vec(todo_list);
        Ok(())
    } else {
        println!("Invalid arguments");
        Ok(())
    }
}

fn display_todo_vec(todo_vec: Vec<Todo>) {
    todo_vec.iter().for_each(|todo| {
        let is_completed = if todo.is_completed == 1 {
            "completed".green()
        } else {
            "incomplete".red()
        };
        println!("{:<20}\t{:^20}\t{:>40}", todo.title, is_completed, todo.get_id());
    });
}
