use actix_web::{App, HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use crate::db::redis::{set_api_key,verify_api_key};
#[post("create-api-key")]
pub async fn create_api_key(){
    
}
#[get("/upload-function")]
pub async fn upload_function(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Upload Function")
}

pub async fn delete_api_key(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Delete API Key")
}
