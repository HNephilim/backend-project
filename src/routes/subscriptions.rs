use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;


#[derive(serde::Deserialize)]
pub struct FormData{
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, conn_pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, conn_pool: web::Data<PgPool>) -> HttpResponse {

    match insert_subscriber(&form, &conn_pool).await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }

}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, conn_pool),
)]
async fn insert_subscriber(form: &FormData, conn_pool: &PgPool) -> Result<(), sqlx::Error>{

    sqlx::query!(
        r#"
        INSERT INTO subscription (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
        .execute(conn_pool)
        .await
        .map_err(|e|{
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?; // The ? operator will return early with and sqxl::Error.

    Ok(())
}
