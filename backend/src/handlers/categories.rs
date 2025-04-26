use crate::models::*;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_categories(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
