use actix_web::{HttpResponse, web};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct DelForm{
    pub email: String,
}

pub async fn delete(form: web::Form<DelForm>, conn_pool: web::Data<PgPool>) -> HttpResponse{

    match sqlx::query!(
        r#"
        DELETE FROM subscription
        WHERE email=$1
        "#,
        form.email
    )
        .execute(conn_pool.get_ref())
        .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}