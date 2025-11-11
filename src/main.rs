mod db;
mod server;
mod utils;
use actix_web::{
    middleware::from_fn,
    {App, HttpServer},
};
use server::middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(from_fn(middleware::verify_api_key)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
