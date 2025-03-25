mod db;
use db::{connect, initalize_db, query_task, query_user, write_task, write_user, get_tasks, NewTask, NewUser};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let connection = connect().await.unwrap();

    println!("====================== INITIALIZING DB ======================");
    initalize_db(&connection).await?;

    println!("====================== CREATING USER ======================");
    let user = NewUser {
        username: String::from("Kam"),
        password_hashed: String::from("kam123")
    };
    println!("Use created successfully");
    println!("====================== WRITING USER ======================");
    let user_id: Uuid = write_user(&connection, &user).await?;

    println!("====================== QUERYING USER ======================");
    query_user(&connection, &user_id).await?;

    println!("====================== CREATING TASK ======================");
    let task = NewTask {
        user_id: user_id,
        task_title: String::from("Temp Title"),
        task_description: None,
        task_status: String::from("in progress")
    };

    let task2 = NewTask {
        user_id: user_id,
        task_title: String::from("Another title"),
        task_description: Some(String::from("Test desc")),
        task_status: String::from("in progress")
    };

    println!("====================== WRITING TASK ======================");
    let task_id = write_task(&connection, &task).await?;
    println!("{}", task_id);
    let task2_id = write_task(&connection, &task2).await?;

    println!("====================== QUERYING TASK ======================");
    query_task(&connection, &task_id).await?;
    query_task(&connection, &task2_id).await?;

    println!("====================== GETTING ALL TASKS FROM USER ======================");
    get_tasks(&connection, &user_id).await?;

    Ok(())
}
