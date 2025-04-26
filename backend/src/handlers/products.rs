use crate::models::*;
use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use futures_util::StreamExt;
use image;
use log::{error, info};
use sqlx::PgPool;
use std::fs;
use std::path::Path;
use uuid::Uuid;

// Product CRUD
pub async fn create_product(
    product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    info!("Received product creation request: {:?}", product);
    let product = product.into_inner();
    // Start a transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(format!("Error: {}", e));
        }
    };
    // Insert the product
    let product_result = match sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (vendor_id, name, description, price, image_url, is_draft)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(product.vendor_id)
    .bind(product.name)
    .bind(product.description)
    .bind(product.price)
    .bind(product.image_url)
    .bind(product.is_draft)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(product) => product,
        Err(e) => {
            error!("Failed to create product: {}", e);
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(format!("Error: {}", e));
        }
    };
    // Insert category if provided
    if let Some(category_id) = product.category_id {
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO product_categories (product_id, category_id)
            VALUES ($1, $2)
            "#,
        )
        .bind(product_result.id)
        .bind(category_id)
        .execute(&mut *tx)
        .await
        {
            error!("Failed to add category: {}", e);
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(format!("Error: {}", e));
        }
    }
    // Insert tags if provided
    if let Some(tag_ids) = product.tag_ids {
        for tag_id in tag_ids {
            if let Err(e) = sqlx::query(
                r#"
                INSERT INTO product_tags (product_id, tag_id)
                VALUES ($1, $2)
                "#,
            )
            .bind(product_result.id)
            .bind(tag_id)
            .execute(&mut *tx)
            .await
            {
                error!("Failed to add tag: {}", e);
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json(format!("Error: {}", e));
            }
        }
    }
    // Commit the transaction
    if let Err(e) = tx.commit().await {
        error!("Failed to commit transaction: {}", e);
        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
    }
    info!("Successfully created product: {:?}", product_result);
    HttpResponse::Created().json(product_result)
}

pub async fn get_products(pool: web::Data<PgPool>) -> impl Responder {
    info!("Fetching all products");
    match sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(products) => {
            info!("Found {} products", products.len());
            HttpResponse::Ok().json(products)
        }
        Err(e) => {
            error!("Failed to fetch products: {}", e);
            HttpResponse::InternalServerError().json(format!("Error: {}", e))
        }
    }
}

pub async fn update_product(
    product_id: web::Path<Uuid>,
    product: Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    let product = product.into_inner();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    // Update the product
    let update_result = sqlx::query(
        r#"
        UPDATE products
        SET vendor_id = $1, name = $2, description = $3, price = $4, image_url = $5, is_draft = $6, updated_at = NOW()
        WHERE id = $7
        "#
    )
    .bind(product.vendor_id)
    .bind(product.name)
    .bind(product.description)
    .bind(product.price)
    .bind(product.image_url)
    .bind(product.is_draft)
    .bind(product_id)
    .execute(&mut *tx)
    .await;
    if let Err(e) = update_result {
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
    }
    // Update category (remove old, add new if provided)
    let _ = sqlx::query("DELETE FROM product_categories WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await;
    if let Some(category_id) = product.category_id {
        let _ = sqlx::query(
            r#"INSERT INTO product_categories (product_id, category_id) VALUES ($1, $2)"#,
        )
        .bind(product_id)
        .bind(category_id)
        .execute(&mut *tx)
        .await;
    }
    // Update tags (remove old, add new if provided)
    let _ = sqlx::query("DELETE FROM product_tags WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await;
    if let Some(tag_ids) = product.tag_ids {
        for tag_id in tag_ids {
            let _ = sqlx::query(r#"INSERT INTO product_tags (product_id, tag_id) VALUES ($1, $2)"#)
                .bind(product_id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await;
        }
    }
    if let Err(e) = tx.commit().await {
        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
    }
    HttpResponse::Ok().json("Product updated successfully")
}

pub async fn delete_product(
    product_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    // Delete from product_categories
    if let Err(e) = sqlx::query("DELETE FROM product_categories WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await
    {
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
    }
    // Delete from product_tags
    if let Err(e) = sqlx::query("DELETE FROM product_tags WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await
    {
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
    }
    // Delete from notifications
    if let Err(e) = sqlx::query("DELETE FROM notifications WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await
    {
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
    }
    // Delete the product
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(product_id)
        .execute(&mut *tx)
        .await;
    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                let _ = tx.rollback().await;
                HttpResponse::NotFound().json("Product not found")
            } else {
                let _ = tx.commit().await;
                HttpResponse::Ok().json("Product deleted successfully")
            }
        }
        Err(e) => {
            let _ = tx.rollback().await;
            HttpResponse::InternalServerError().json(format!("Error: {}", e))
        }
    }
}

// Draft and submit
pub async fn save_draft(product_id: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let product_id = product_id.into_inner();
    match sqlx::query(
        r#"
        UPDATE products
        SET is_draft = true
        WHERE id = $1
        "#,
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
    // Set is_verified to NULL (pending)
    let _ =
        sqlx::query(r#"UPDATE products SET is_draft = false, is_verified = NULL WHERE id = $1"#)
            .bind(product_id)
            .execute(pool.get_ref())
            .await;
    // Trigger verification process
    verify_product(product_id, pool).await;
    HttpResponse::Ok().json("Product submitted successfully")
}

async fn verify_product(product_id: Uuid, pool: web::Data<PgPool>) {
    // Fetch product and related info
    let product = sqlx::query!(
        r#"
        SELECT p.name, p.description, p.price, p.image_url,
            (SELECT COUNT(*) FROM product_categories WHERE product_id = p.id) as category_count,
            (SELECT COUNT(*) FROM product_tags WHERE product_id = p.id) as tag_count,
            p.vendor_id
        FROM products p
        WHERE p.id = $1
        "#,
        product_id
    )
    .fetch_one(pool.get_ref())
    .await;

    let (is_verified, message) = match product {
        Ok(p) => {
            let has_name = !p.name.trim().is_empty();
            let has_desc = !p.description.trim().is_empty();
            let has_image = !p.image_url.trim().is_empty();
            let has_category = p.category_count.unwrap_or(0) > 0;
            let has_tag = p.tag_count.unwrap_or(0) > 0;
            let price_ok = p.price >= 500.0 && p.price <= 1_000_000_000.0;
            if has_name && has_desc && has_image && has_category && has_tag && price_ok {
                (true, "Your product has been verified and is now live!")
            } else {
                (
                    false,
                    "Your product verification failed. Please check the requirements.",
                )
            }
        }
        Err(_) => (
            false,
            "Your product verification failed. Please check the requirements.",
        ),
    };

    // Update verification status
    let _ = sqlx::query(r#"UPDATE products SET is_verified = $1 WHERE id = $2"#)
        .bind(is_verified)
        .bind(product_id)
        .execute(pool.get_ref())
        .await;

    // Create notification
    match sqlx::query(
        r#"
        INSERT INTO notifications (vendor_id, product_id, message)
        SELECT vendor_id, id, $1
        FROM products
        WHERE id = $2
        "#
    )
    .bind(message)
    .bind(product_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {},
        Err(e) => error!("Failed to insert notification: {}", e),
    }
}

// File upload
pub async fn upload_file(mut payload: Multipart) -> impl Responder {
    let upload_dir = Path::new("uploads");
    if !upload_dir.exists() {
        if let Err(e) = fs::create_dir_all(upload_dir) {
            error!("Failed to create upload directory: {}", e);
            return HttpResponse::InternalServerError().json(format!("Error: {}", e));
        }
    }
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                error!("Error in multipart field: {}", e);
                return HttpResponse::BadRequest().json(format!("Error: {}", e));
            }
        };
        let content_type = field.content_type().map(|ct| ct.clone());
        if let Some(ct) = content_type {
            if ct.type_() == "image" {
                let ext = ct.subtype().as_str();
                let filename = format!("{}.{}", Uuid::new_v4(), ext);
                let filepath = upload_dir.join(&filename);
                // Read the image into memory
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(data) => data,
                        Err(e) => {
                            error!("Error reading chunk: {}", e);
                            return HttpResponse::BadRequest().json(format!("Error: {}", e));
                        }
                    };
                    bytes.extend_from_slice(&data);
                }
                // Try to optimize JPEG and PNG
                let optimized = match ext {
                    "jpeg" | "jpg" | "png" => match image::load_from_memory(&bytes) {
                        Ok(img) => {
                            let resized =
                                img.resize(800, 800, image::imageops::FilterType::Lanczos3);
                            let mut out = Vec::new();
                            if ext == "png" {
                                let mut cursor = std::io::Cursor::new(&mut out);
                                if resized
                                    .write_to(&mut cursor, image::ImageOutputFormat::Png)
                                    .is_ok()
                                {
                                    Some(out)
                                } else {
                                    None
                                }
                            } else {
                                let mut cursor = std::io::Cursor::new(&mut out);
                                if resized
                                    .write_to(&mut cursor, image::ImageOutputFormat::Jpeg(70))
                                    .is_ok()
                                {
                                    Some(out)
                                } else {
                                    None
                                }
                            }
                        }
                        Err(_) => None,
                    },
                    _ => None,
                };
                // Save the optimized or original image
                let to_save = optimized.as_ref().unwrap_or(&bytes);
                match fs::write(&filepath, to_save) {
                    Ok(_) => {
                        let image_url = format!("/uploads/{}", filename);
                        return HttpResponse::Ok().json(serde_json::json!({
                            "success": true,
                            "image_url": image_url
                        }));
                    }
                    Err(e) => {
                        error!("Failed to save file: {}", e);
                        return HttpResponse::InternalServerError().json(format!("Error: {}", e));
                    }
                }
            }
        }
    }
    HttpResponse::BadRequest().json(serde_json::json!({
        "success": false,
        "error": "No image file found in the request"
    }))
}
