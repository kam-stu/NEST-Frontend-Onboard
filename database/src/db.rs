use sqlx::{pool, Executor, FromRow, PgPool};
use std::fs;
use tokio::time::{timeout, Duration};
use sqlx::types::Uuid;
use chrono::NaiveDateTime;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,          
    pub username: String,
    pub password_hashed: String,
}

#[derive(Debug, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_title: String,
    pub task_description: Option<String>,
    pub creation_time: NaiveDateTime,
    pub task_status: String,
    pub completion_time: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub password_hashed: String,
}

#[derive(Debug)]
pub struct NewTask {
    pub user_id: Uuid,
    pub task_title: String,
    pub task_description: Option<String>,
    pub task_status: String,
}


// this will later be moved to backend; just wanted to test the connection
// without setting up the backend folder
pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let url: &str = "postgres://root:root@localhost:5432/postgres";
    println!("Running...");

    // connects to the databse.  It tires to connect for 5 seconds and if there's an error
    //  or no response, it throws the error (or conenction timeout) and exits the rest of the program
    let pool = match timeout(Duration::from_secs(5), async { PgPool::connect(url).await }).await {
        Ok(Ok(pool)) => {
            println!("Connection established!");
            pool
        }
        Ok(Err(e)) => {
            eprintln!("Connection failed: {:?}", e);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Connection Timeout.");
            std::process::exit(1);
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

    match pool.execute(schema.as_str()).await {
        Ok(_) => {
            println!("Database initialized succeffully!");
            Ok(())
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("42P07") => {
            println!("Database is alerady initialized.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Database initialization failed: {:?}", e);
            Err(e)
        }
    }
}

// inserts a new user into the database
pub async fn write_user(pool: &PgPool, user: &NewUser) -> Result<Uuid, sqlx::Error> {
    let result: (Uuid,) = sqlx::query_as(
        "INSERT INTO users (username, password_hashed)
        VALUES ($1, $2)
        RETURNING id")
        .bind(&user.username)
        .bind(&user.password_hashed)
        .fetch_one(pool)
        .await?;

    println!("User added successfully");

    Ok(result.0)
}

// checks if a username already exists by counting all users with a specific username
// returns true if the count is greather than 0 (someone already has that username)
pub async fn username_exists(pool: &PgPool, username: &String) -> bool {
    let exists: (i8, ) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE username=$1")
        .bind(&username)
        .fetch_one(pool)
        .await.unwrap();

    exists.0 > 0
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
pub async fn get_user_uuid(pool: &PgPool, username: &String) -> Result<Uuid, sqlx::Error> {
    let result: (Uuid,) = sqlx::query_as(
        "SELECT id FROM users WHERE username=$1")
        .bind(&username)
        .fetch_one(pool)
        .await?;

        println!("User ID found successfully!");
    Ok(result.0)
}
*/

// sets a task into the database -> returns the id of that task
pub async fn write_task(pool: &PgPool, task: &NewTask) -> Result<Uuid, sqlx::Error> {
    let result : (Uuid,)= sqlx::query_as(
        "INSERT INTO tasks (user_id, task_title, task_description, task_status)
        VALUES ($1, $2, $3, $4)
        RETURNING id")
        .bind(&task.user_id)
        .bind(&task.task_title)
        .bind(&task.task_description)
        .bind(&task.task_status)
        .fetch_one(pool)
        .await?;

    Ok(result.0)
}

// gets a task based on its id
pub async fn query_task(pool: &PgPool, task_id: &Uuid) -> Result<Task, sqlx::Error> {
    let result: Task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id=$1")
        .bind(&task_id)
        .fetch_one(pool)
        .await?;
    
    println!("The task is: {:?}", result);

    Ok(result)    
}


// returns a vector of the tasks associated with a user 
pub async fn get_tasks(pool: &PgPool, user_id: &Uuid) -> Result<Vec<Task>, sqlx::Error> {
    let result = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE user_id=$1").
        bind(&user_id)
        .fetch_all(pool)
        .await?;

    println!("All tasks from {:?}: {:?}", query_user(pool, &user_id).await?, result);

    Ok(result)
}

// updates the task of a program 
pub async fn edit_task(pool: &PgPool, task: &Task) -> Result<(), sqlx::Error>{
    sqlx::query_as::<_, Task>(
        "UPDATE tasks
        SET task_title=$1, task_description=$2, task_status=$3
        WHERE id=$4")
        .bind(&task.task_title)
        .bind(&task.task_description)
        .bind(&task.task_status)
        .bind(&task.id)
        .fetch_one(pool)
        .await?;

    Ok(())
}

// updates the username and/or password of a user
pub async fn edit_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query_as::<_, User>(
        "UPDATE users
        SET username=$1, password_hashed=$2
        WHERE id=$3")
        .bind(&user.username)
        .bind(&user.password_hashed)
        .bind(&user.id)
        .fetch_one(pool)
        .await?;

        Ok(())
}


