use duckdb::Connection;
use duckdb::Result;
pub fn connect_file_db() -> Result<Connection> {
    let conn = Connection::open("./database/crab_lamda.db")?;
    Ok(conn)
}
