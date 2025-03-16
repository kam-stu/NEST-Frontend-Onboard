mod db;
use db::connect;

// tests the connection for database
fn main() {
    let _err = connect();
}
