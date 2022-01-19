use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};


#[derive(serde::Deserialize)]
pub struct FormData{
    pub email: String,
    pub name: String,
}

impl TryFrom<FormData> for NewSubscriber{
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self{email,name})
    }
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
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    match insert_subscriber(&new_subscriber, &conn_pool).await
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
    skip(new_subscriber, conn_pool),
)]
async fn insert_subscriber(new_subscriber: &NewSubscriber, conn_pool: &PgPool) -> Result<(), sqlx::Error>{

    sqlx::query!(
        r#"
        INSERT INTO subscription (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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
