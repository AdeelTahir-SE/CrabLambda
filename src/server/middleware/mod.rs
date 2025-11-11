use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpResponse, body::BoxBody,
};

// Function-based middleware
pub async fn verify_api_key(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error>

{
    // Extract headers
    let api_key_header = req.headers().get("crablambda-api-key");
    let user_id_header = req.headers().get("crablambda-user-id");

    // Verify headers exist and match
    let authorized = match (api_key_header, user_id_header) {
        (Some(api_key), Some(user_id)) => {
            // Convert HeaderValue to str safely
            match (api_key.to_str(), user_id.to_str()) {
                (Ok(api_key_str), Ok(user_id_str)) => api_key_str == user_id_str,
                _ => false, // if headers are not valid UTF-8
            }
        }
        _ => false, // missing headers
    };

    if authorized {
        // Continue to next middleware or handler
        let res = next.call(req).await?;
        Ok(res)
    } else {
        // Unauthorized response
        let (req, _) = req.into_parts();
        let resp = HttpResponse::Unauthorized().finish().map_into_boxed_body();
        Ok(ServiceResponse::new(req, resp))
    }
}
