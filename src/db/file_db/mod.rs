pub mod insert_file_functions;
pub mod read_file_functions;
pub mod update_file_functions;
pub mod delete_file_functions;
pub mod create_file_functions;
use dotenvy::dotenv;
use duckdb::{Connection,Result};

pub fn connect_file_db() -> Result<Connection> {
    dotenv().ok();
    let conn = Connection::open(std::env::var("DATABASE_URL").unwrap())?;
    Ok(conn)
}

