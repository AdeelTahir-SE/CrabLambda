use duckdb::{Connection, Result, params};

pub fn read_table_function(
    conn: &Connection,
    id: &str,
) -> Result<Option<(String, String, String, String, String, String,String,String,String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, code,language, create_time, update_time, port, user_id,dependencies FROM Function WHERE id = ?",
    )?;
    let rows = stmt.query_map(params![id], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
        ))
    })?;

    for row in rows {
        return Ok(Some(row?)); //returns the first row
    }
    
    Ok(None)
}