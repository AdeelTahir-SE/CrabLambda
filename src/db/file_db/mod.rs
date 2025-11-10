pub mod insert_file_functions;
pub mod read_file_functions;
pub mod update_file_functions;
pub mod delete_file_functions;

use duckdb::{Connection,Result};


pub fn connect_file_db() -> Result<Connection> {
    let conn = Connection::open("./database/crab_lamda.db")?;
    Ok(conn)
}
