use sqlx::{Executor, FromRow, PgPool};
use std::fs;
use tokio::time::{timeout, Duration};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,          
    pub username: String,
    pub password_hashed: String,
}

pub struct Task {
    pub id: Uuid,                     
    pub user_id: Uuid,                 
    pub title: String,                 
    pub description: Option<String>,   
    pub creation_time: DateTime<Utc>,  
    pub status: String,                
    pub completion_time: Option<DateTime<Utc>>,
}


// this will later be moved to backend; just wanted to test the connection
// without setting up the backend folder
#[tokio::main]
pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let url: &str = "postgres://root:root@localhost:5432/postgres";
    println!("Running...");

    // connection to database
    // if fails to connect within 5 seconds, it stops and sends a connection timeout error
    // otherwise, establishes connection properly or sends a different error
    let pool = match timeout(Duration::from_secs(5), async { PgPool::connect(url).await }).await {
        Ok(Ok(pool)) => {
            println!("Connection established!");
            pool
        }
        Ok(Err(e)) => {
            eprintln!("Connection failed: {:?}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Connection Timeout.");
            return Err(sqlx::Error::Configuration("Connection timeout".into()));
        }
    };
    Ok(pool)

    // here is other solution, but it hangs on connection timeout
    // keeps the PgPool type though...not entirely sure if that's necessary yet...
    /*
        let pool = PgPool::connect(url).await?;
     */
}

pub async fn initalize_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    let schema = fs::read_to_string("./schema.sql")?;

    pool.execute(schema.as_str()).await?;

    println!("Database initialized successfully!");
    Ok(())
}

// inserts a new user into the database and returns their uuid
pub async fn write_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, password_hashed)
        VALUES ($1, $2, $3)
        RETURNING id")
        .bind(&user.id).bind(&user.username).bind(&user.password_hashed).fetch_one(pool).await?;

    println!("User added successfully");

    Ok(())
}

// gets a user from the databse from their ID and prints the values associated with that user 
pub async fn query_user(pool: &PgPool, user_id: &Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hashed FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_one(pool)
        .await?;
    println!("The user is: {:?}", result);

    Ok(())
}

/* 

// gets a task and user, inserts the task into the database and sets its owner as the user 
// returns the task's uuid
pub async fn write_task(pool: &PgPool, task: &Task, user_id: Uuid) -> Result<Uuid, sqlx::Error> {}

// returns a vector of the tasks associated with a user 
pub async fn get_task(pool: &PgPool, user_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {}

// sets the tasks owner to the user's UUID
pub async fn set_task_owner(pool: &PgPool, task_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {}

*/