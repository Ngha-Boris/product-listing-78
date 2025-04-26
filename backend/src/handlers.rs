use crate::models::*;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_categories(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn get_tags(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Tag>("SELECT * FROM tags")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn create_product(
    product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product = product.into_inner();
    
    match sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (vendor_id, name, description, price, image_url, is_draft)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(product.vendor_id)
    .bind(product.name)
    .bind(product.description)
    .bind(product.price)
    .bind(product.image_url)
    .bind(product.is_draft)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn save_draft(
    product_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    match sqlx::query(
        r#"
        UPDATE products
        SET is_draft = true
        WHERE id = $1
        "#
    )
    .bind(product_id)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json("Draft saved successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn submit_product(
    product_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    match sqlx::query(
        r#"
        UPDATE products
        SET is_draft = false
        WHERE id = $1
        "#
    )
    .bind(product_id)
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            // Trigger verification process
            verify_product(product_id, pool).await;
            HttpResponse::Ok().json("Product submitted successfully")
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn verify_product(product_id: Uuid, pool: web::Data<PgPool>) {
    // Here you would implement your verification logic
    // For now, we'll just simulate it
    let is_verified = true; // This would be determined by your verification process
    
    let _ = sqlx::query(
        r#"
        UPDATE products
        SET is_verified = $1
        WHERE id = $2
        "#
    )
    .bind(is_verified)
    .bind(product_id)
    .execute(pool.get_ref())
    .await;

    // Create notification
    let _ = sqlx::query(
        r#"
        INSERT INTO notifications (vendor_id, product_id, message)
        SELECT vendor_id, $1, $2
        FROM products
        WHERE id = $1
        "#
    )
    .bind(product_id)
    .bind(if is_verified {
        "Your product has been verified and is now live!"
    } else {
        "Your product verification failed. Please check the requirements."
    })
    .execute(pool.get_ref())
    .await;
}

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
        "#
    )
    .bind(vendor_id)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
} 