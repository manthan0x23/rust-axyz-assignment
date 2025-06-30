use actix_web::{ Responder, get};
use crate::utils::t::ApiResponse;

#[get("/test-api")]
pub async fn test() -> impl Responder {
    println!("Test is here ");
    ApiResponse::success("API is live")
}
