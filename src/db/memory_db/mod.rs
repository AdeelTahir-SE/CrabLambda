pub mod insert_memory_functions;
pub mod read_memory_functions;
pub mod update_memory_functions;
pub mod delete_memory_functions;


use duckdb::{Connection,Result};


pub fn connect_memory_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    Ok(conn)
}
