use  duckdb::{Connection,Result};

//creates table User
pub fn create_table_user(connection: &Connection) -> Result<()> {
    let sql = "
    CREATE TABLE IF NOT EXISTS User (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL ,
        password TEXT NOT NULL,
        email TEXT NOT NULL ,
        create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    ";
    connection.execute(sql, [])?;
    Ok(())
}