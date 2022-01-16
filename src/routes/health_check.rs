use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse{
    HttpResponse::Ok().body("I'm working <3 <br> And deployed automatic by the Master Erick")
}