use crate::{container::dockerizer::initialize_container_deploayment, db::redis};
use redis::{delete_api_key, set_api_key, verify_api_key};
use crate::utils::uuid_generator::generate_random_uid;
use crate::db::file_db;
use file_db::insert_file_functions::insert_table_function; 
use file_db::read_file_functions::read_table_function;
use file_db::connect_file_db;
use actix_web::{App, HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateApiKeyRequest {
    user_id: String,
}

#[derive(serde::Serialize)]
struct CreateApiKeyResponse {
    api_key: String,
    message: String,
}


#[derive(Deserialize)]
struct UploadFunctionRequest {
    name: String,
    code: String,
    language: String,
    dependencies: String,
}

#[derive(Serialize)]
struct UploadFunctionResponse {
    function_id: String,
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


#[post("/upload-function")]
pub async fn upload_function(req: HttpRequest, data: web::Json<UploadFunctionRequest>) -> impl Responder {
    let user_id = req.headers().get("crablambda-user-id").unwrap().to_str().unwrap();
    let name = &data.name;
    let code = &data.code;
    let language = &data.language;
    let dependencies = &data.dependencies;

    let function_id = generate_random_uid();
    let connection = connect_file_db().unwrap();
    let res= insert_table_function(&connection, &function_id, &name, &code,&language, &user_id,&dependencies);

    match res {
        Ok(_) => HttpResponse::Ok().json(UploadFunctionResponse { function_id }),
        _ => HttpResponse::BadRequest().body("Failed to Upload Function"),
    };

    return HttpResponse::BadRequest().body("Failed to Upload Function");

}


#[get("/request-function")]
pub async fn request_function(req: HttpRequest) -> impl Responder {
    let function_id = req.headers().get("crablambda-function-id").unwrap().to_str().unwrap();
    let connection = connect_file_db().unwrap();
    
    let function_row=read_table_function(&connection, function_id);
    let row = function_row;

    match row{
        Ok(row) => {
            let port=row.unwrap_or_default().6.parse::<u16>().unwrap();
            let language=row.unwrap_or_default().3;
            let dependencies=row.unwrap_or_default().8;
            let dependencies: Vec<&str> = serde_json::from_str(&dependencies).unwrap_or_default();
            let function=row.unwrap_or_default().2;
            if port == 0{
                //deploy container
                initialize_container_deploayment(&language, &function, dependencies);
                

            }
            else{
                // forward request

            }
        }
        Err(_) => HttpResponse::BadRequest().body("Failed to Execute the Read Function"),
    }
   
}

#[post("/request-function")]
pub async fn request_function(req::HttpRequest)->impl Responder{
    
}
#[put("/request-function")]
pub async fn request_function(req::HttpRequest)->impl Responder{

}
#[delete("/request-function")]
pub async fn request_function(req::HttpRequest)->impl Responder{

}

