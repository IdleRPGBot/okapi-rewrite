use actix_web::{get, HttpResponse};

#[get("/")]
async fn index() -> HttpResponse {
    // For metrics
    HttpResponse::Ok().content_type("text/plain").body("1")
}
