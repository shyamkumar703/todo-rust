pub enum Env {
    Prod,
    Test,
}

impl Env {
    pub fn tbl_name(&self) -> &str {
        match self {
            Env::Prod => "prod",
            Env::Test => "test",
        }
    }
}