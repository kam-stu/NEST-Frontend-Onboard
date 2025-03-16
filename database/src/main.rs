mod db;
use db::{connect, initalize_db};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // redundant pool; ignores the Result<PgPool> type error that db.connect() has at the moment
    let pool = sqlx::PgPool::connect("postgres://root:root@localhost:5432/postgres").await?;
    initalize_db(&pool).await?;

    Ok(())
}
