use duckdb::Connection;
use duckdb::Result;

pub fn connect_memory_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    Ok(conn)
}
