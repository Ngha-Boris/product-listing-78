use crate::models::*;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_tags(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Tag>("SELECT * FROM tags")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
