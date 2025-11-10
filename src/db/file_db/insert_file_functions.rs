use duckdb::{params,Connection,Result};

pub fn insert_file_functions_data(conn:&Connection,name:&str,sql:&str) -> Result<()> {
    conn.execute(
        "INSERT INTO file_functions (name, sql) VALUES (?, ?)",
        params![name, sql],
    )?;
    Ok(())
}
