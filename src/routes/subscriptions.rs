use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe_route(
    form: web::Form<FormData>,
    db_conn_pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    log::info!(
        "request_id {} - Saving '{}' '{}' as a new subscriber in PostgreSQL",
        request_id,
        form.name,
        form.email
    );

    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_conn_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "request_id {} - Saved new subscriber details in PostgreSQL",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            log::info!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
