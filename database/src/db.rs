use sqlx::PgPool;
use tokio::time::{timeout, Duration};

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
        let pool = PgPool::conect(url).await?;
     */
}