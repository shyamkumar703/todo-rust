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

    pub async fn list_all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(self.env.tbl_name()).await?;
        let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at").fetch_all(&db).await?;

        Ok(result)
    }

    pub async fn mark_as_complete(&self, id: &String) -> Result<(), Box<dyn std::error::Error>> {
        let now = chrono::offset::Utc::now().timestamp_millis();
        let db = SqlitePool::connect(self.env.tbl_name()).await?;
        sqlx::query("UPDATE todos SET is_completed = 1, updated_at = $1 WHERE id = $2").bind(now).bind(id).execute(&db).await?;

        Ok(())
    }

    pub async fn get_recent_todos(&self, limit: u32) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(self.env.tbl_name()).await?;
        let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC LIMIT $1").bind(limit).fetch_all(&db).await?;

        Ok(result)
    }

    pub async fn get_incomplete_todos(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(self.env.tbl_name()).await?;
        let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE is_completed=0 ORDER BY created_at").fetch_all(&db).await?;

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

    #[tokio::test]
    async fn test_list_todo_does_not_panic() {
        let db = Db::new(Env::Test).await.expect("Could not get db");
        let id = Uuid::new_v4().to_string();
        let todo = Todo::new(id, "test".into(), false, 0, 0);
        db.insert(&todo).await.expect("Could not insert todo");
        let todo_vec = db.list_all().await.expect("Could not list all todos");
        assert!(todo_vec.iter().count() > 0);
        assert!(todo_vec.iter().filter(|todo_filter| todo_filter.id == todo.id).collect::<Vec<&Todo>>().len() > 0);
    }

    #[tokio::test]
    async fn test_mark_as_complete_works() {
        let db = Db::new(Env::Test).await.expect("Could not get db");
        let id = Uuid::new_v4().to_string();
        let todo = Todo::new(id, "test".into(), false, 0, 0);
        db.insert(&todo).await.expect("Could not insert todo");
        db.mark_as_complete(&todo.id).await.expect("Could not mark todo as complete");
        let todo_updated = db.get(&todo.id).await.expect("Could not get updated todo");
        assert_eq!(todo_updated.is_completed, 1);
        assert_ne!(todo.updated_at, todo_updated.updated_at);

    }

    #[tokio::test]
    async fn test_get_recent_todos() {
        let now = chrono::offset::Utc::now().timestamp_millis();
        let db = Db::new(Env::Test).await.expect("Could not get db");

        let id1 = Uuid::new_v4().to_string();
        let todo1 = Todo::new(id1, "test".into(), false, now, now);
        db.insert(&todo1).await.expect("Could not insert todo");

        let id2 = Uuid::new_v4().to_string();
        let todo2 = Todo::new(id2, "test".into(), false, now, now);
        db.insert(&todo2).await.expect("Could not insert todo");

        let recent_todos = db.get_recent_todos(2).await.expect("Could not get recent todos");
        assert!(recent_todos.iter().filter(|todo| todo.id == todo1.id || todo.id == todo2.id).count() == 2);
    }

    #[tokio::test]
    async fn test_get_incomplete_todos() {
        let db = Db::new(Env::Test).await.expect("Could not get db");
        let todo = Todo::new(
            Uuid::new_v4().to_string(),
            "test".into(),
            false,
            0,
            0
        );
        db.insert(&todo).await.expect("Could not insert todo");

        let incomplete_todos = db.get_incomplete_todos().await.expect("Could not get incomplete todos");
        assert!(incomplete_todos.iter().filter(|todo_filter| todo_filter.id == todo.id).count() == 1);

        db.mark_as_complete(&todo.id).await.expect("Could not mark todo as complete");

        let incomplete_todos = db.get_incomplete_todos().await.expect("Could not get incomplete todos");
        assert!(incomplete_todos.iter().filter(|todo_filter| todo_filter.id == todo.id).count() == 0);
    }
}