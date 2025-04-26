mod db;
mod handlers;
mod models;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use handlers::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::create_pool().await.expect("Failed to create pool");
    db::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_headers(["content-type", "content-length"])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .route("/categories", web::get().to(get_categories))
                    .route("/tags", web::get().to(get_tags))
                    .route("/products", web::get().to(get_products))
                    .route("/products", web::post().to(create_product))
                    .route("/products/{id}", web::delete().to(delete_product))
                    .route("/products/{id}", web::put().to(update_product))
                    .route("/upload", web::post().to(upload_file))
                    .route("/products/{id}/draft", web::post().to(save_draft))
                    .route("/products/{id}/submit", web::post().to(submit_product))
                    .route(
                        "/vendors/{id}/notifications",
                        web::get().to(get_notifications),
                    ),
            )
            .service(Files::new("/uploads", "uploads"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
