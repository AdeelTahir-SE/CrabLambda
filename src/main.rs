mod db;
use db::file_db::{connect_file_db, insert_file_functions};
fn main() {
    let connection = connect_file_db().unwrap();
    insert_file_functions::insert_file_functions_data(&connection, "test", "select 1").unwrap();
}
