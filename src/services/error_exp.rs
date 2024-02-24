//! This module is used to explore the best practice of ergonomic error handling.

use actix_web::{get, Responder, web};

// Use the error helpers provided by actix_web will fulfill the body of response with plain text.
#[get("/bad-request")]
async fn bad_request() -> impl Responder {
    Err::<String, _>(actix_web::error::ErrorBadRequest("Hello, world!"))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/error-exp").service(bad_request));
}