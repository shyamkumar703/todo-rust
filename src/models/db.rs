use crate::models::env::Env;
use crate::models::todo::Todo;
use sqlx::{ migrate::MigrateDatabase, Sqlite, SqlitePool };

pub struct Db {
    env: Env
}

impl Db {
    pub async fn new(env: Env) -> Result<Self, Box<dyn std::error::Error>> {
        let tbl_name = env.tbl_name();

        if !Sqlite::database_exists(tbl_name).await.unwrap_or(false) {
            Sqlite::create_database(tbl_name).await?;
        }

        let db = SqlitePool::connect(tbl_name).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS todos (id TEXT PRIMARY KEY, title TEXT, is_completed INTEGER, created_at INTEGER, updated_at INTEGER);").execute(&db).await?;


        Ok(Self { env })
    }

    pub async fn insert(&self, todo: &Todo) -> Result<(), Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(self.env.tbl_name()).await?;
        todo.sql_insert_query().execute(&db).await?;

        Ok(())
    }

    pub async fn get(&self, id: &String) -> Result<Todo, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(self.env.tbl_name()).await?;
        let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id =?").bind(id).fetch_one(&db).await?;

        Ok(result)

    }
}

pub enum DbError {
    ConnectionError,
    TableCreationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_initialize_creates_table() {
        Db::new(Env::Test).await.expect("Could not create db");
    }

    #[tokio::test]
    async fn test_insert_todo_does_not_panic() {
        let db = Db::new(Env::Test).await.expect("Could not get db");
        let id = Uuid::new_v4().to_string();
        let todo = Todo::new(id, "test".into(), false, 0, 0);
        db.insert(&todo).await.expect("Could not insert");
        let _todo = db.get(todo.get_id()).await.expect("Could not retrieve todo after insert");
    }
}