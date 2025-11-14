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
pub fn create_table_function(connection: &Connection) -> Result<()> {
    let sql = "
    CREATE TABLE IF NOT EXISTS 'Function' (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        code TEXT NOT NULL,
        language TEXT NOT NULL DEFAULT 'node',
        dependencies TEXT DEFAULT '[]',
        create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        user_id TEXT NOT NULL,
        port INTEGER NOT NULL DEFAULT 0,
        FOREIGN KEY (user_id) REFERENCES User(id)
    );
    ";
    connection.execute(sql, [])?;
    Ok(())
}