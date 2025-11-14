use crate::utils::uuid_generator::generate_random_uid;
use duckdb::{Connection, Result, params};
use serde_json::json;

pub fn insert_table_user(conn: &Connection, name: &str, password: &str, email: &str) -> Result<()> {
    let uuid = generate_random_uid();
    conn.execute(
        "INSERT INTO User (id,username, password,email) VALUES (?, ?,?,?)",
        params![uuid, name, password, email],
    )?;
    Ok(())
}

pub fn insert_table_function(
    conn: &Connection,
    id: &str,
    name: &str,
    code: &str,
    language: &str,
    user_id: &str,
    dependencies: &str,
) -> Result<()> {
    let dependencies=serde_json::to_string(&dependencies.split_whitespace().collect::<Vec<&str>>()).unwrap_or_else(|_| "[]".to_string());
    conn.execute(
        "INSERT INTO Function (id,name, code,language,user_id,dependencies) VALUES (?, ?,?,?,?,?)",
        params![id, name, code,language, user_id,&dependencies],
    )?;
    Ok(())
}
