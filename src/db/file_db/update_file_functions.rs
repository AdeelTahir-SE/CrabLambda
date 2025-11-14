use super::connect_file_db;
use duckdb::{self, params,Connection};

pub fn update_table_function_port_no(connection:&Connection,function_id: &str, port_no: u16) -> Result<(), duckdb::Error> {
    connection.execute("UPDATE Function SET port = ? WHERE id = ?",params![port_no, function_id])?;
    Ok(())
}