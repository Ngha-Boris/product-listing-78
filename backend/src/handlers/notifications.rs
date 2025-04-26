use crate::models::*;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_notifications(
    vendor_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let vendor_id = vendor_id.into_inner();
    match sqlx::query_as::<_, Notification>(
        r#"
        SELECT * FROM notifications
        WHERE vendor_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(vendor_id)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
