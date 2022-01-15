use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse{
    HttpResponse::Ok().body("I'm working <3, for a test")
}