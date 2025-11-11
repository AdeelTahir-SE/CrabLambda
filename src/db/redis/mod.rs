use dotenvy::dotenv;
use redis::{Commands, RedisError};

fn get_redis_url() -> String {
    dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    format!("{}:{}", redis_url, port)
}


pub fn set_api_key(api_key: &str, user_id: &str) -> Result<(), redis::RedisError> {
    let redis_url = get_redis_url();
    let client = redis::Client::open(redis_url)?;   // create client
    let mut conn = client.get_connection()?;        // get a connection
    let _: () = conn.set(api_key, user_id)?;                   // now we can set
    Ok(())
}


pub fn verify_api_key(api_key: &str, user_id: &str) -> Result<bool,RedisError> {
    let redis_url = get_redis_url();
    let client = redis::Client::open(redis_url)?;   // create client
    let mut conn = client.get_connection()?;        // get a connection
    let stored_user_id: String = conn.get(api_key)?;
    Ok(stored_user_id == user_id)
}
