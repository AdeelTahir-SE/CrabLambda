use crate::db::redis::{delete_api_key, set_api_key, verify_api_key};
use crate::utils::uuid_generator::generate_random_uid;
use actix_web::{App, HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateApiKeyRequest {
    user_id: String,
}

#[derive(serde::Serialize)]
struct CreateApiKeyResponse {
    api_key: String,
    message: String,
}


#[post("create-api-key")]
pub async fn create_api_key(data: web::Json<CreateApiKeyRequest>) -> impl Responder {
    let user_id = &data.user_id;
    match delete_api_key(&user_id) {
        Ok(_) => {
            let api_key = generate_random_uid();
            match set_api_key(&user_id, &api_key) {
                Ok(_) => HttpResponse::Ok().json(CreateApiKeyResponse {
                    api_key,
                    message: "API Key Created".to_string(),
                }),
                Err(_) => HttpResponse::BadRequest().body("Failed to Set API Key"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Failed to Delete API Key"),
    }
}


#[get("/upload-function")]
pub async fn upload_function(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Upload Function")
}

