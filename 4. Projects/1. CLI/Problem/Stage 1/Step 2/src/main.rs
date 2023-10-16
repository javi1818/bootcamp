mod models;
mod db;
use crate::db::Database;

fn main() {
    let db_file_path = "data/db.json".to_string();
    let file_db = db::JSONFileDatabase {
        file_path: db_file_path
    };

    let db_state = file_db.read_db();

    println!("Welcome To My-Jira!");

    println!("{db_state:?}");
}
