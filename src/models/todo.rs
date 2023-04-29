use sqlx::{query::Query, sqlite::SqliteArguments, FromRow, Sqlite};

#[derive(Clone, FromRow, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub is_completed: u8,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Todo {
    pub fn new(
        id: String,
        title: String,
        is_completed: bool,
        created_at: i64,
        updated_at: i64,
    ) -> Todo {
        println!("{}", id);
        Self {
            id,
            title,
            is_completed: if is_completed { 1 } else { 0 },
            created_at,
            updated_at,
        }
    }

    pub fn sql_insert_query(&self) -> Query<Sqlite, SqliteArguments> {
        sqlx::query("INSERT INTO todos (id, title, is_completed, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(&self.id)
        .bind(&self.title)
        .bind(&self.is_completed)
        .bind(&self.created_at)
        .bind(&self.updated_at)
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}
