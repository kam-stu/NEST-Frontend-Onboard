mod db;
use db::{User, Task, initalize_db, write_user, query_user, get_user_uuid};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // redundant pool; ignores the Result<PgPool> type error that db.connect() has at the moment
    let pool = sqlx::PgPool::connect("postgres://root:root@localhost:5432/postgres").await?;

    println!("====================== INITIALIZING DB ======================");
    initalize_db(&pool).await?;

    println!("====================== CREATING USER ======================");
    let user = User {
        id: Uuid::new_v4(),
        username: String::from("Kam"),
        password_hashed: String::from("kam123")
    };
    println!("Use created successfully");
    println!("====================== WRITING USER ======================");
    write_user(&pool, &user).await?;

    println!("===================== FINDING USER_ID =====================");
    let user_id = get_user_uuid(&pool, &user.username).await?;

    println!("====================== QUERYING USER ======================");
    query_user(&pool, &user_id).await?;
    
    Ok(())
}
