use duckdb::{params,Connection,Result};
use crate::utils::uuid_generator::generate_random_uid;
pub fn insert_table_user(conn:&Connection,name:&str,password:&str,email:&str) -> Result<()> {
    let uuid=generate_random_uid();
    conn.execute(
        "INSERT INTO User (id,username, password,email) VALUES (?, ?,?,?)",
        params![uuid,name, password,email],
    )?;
    Ok(())
}
