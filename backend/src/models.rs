use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String,
    pub is_draft: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub vendor_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String,
    pub is_draft: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCategory {
    pub product_id: Uuid,
    pub category_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductTag {
    pub product_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub product_id: Uuid,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
} 