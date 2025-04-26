use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS vendors (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS categories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS tags (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            vendor_id UUID NOT NULL REFERENCES vendors(id),
            name VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            price DECIMAL(10,2) NOT NULL,
            image_url TEXT NOT NULL,
            is_draft BOOLEAN DEFAULT true,
            is_verified BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS product_categories (
            product_id UUID REFERENCES products(id),
            category_id UUID REFERENCES categories(id),
            PRIMARY KEY (product_id, category_id)
        );

        CREATE TABLE IF NOT EXISTS product_tags (
            product_id UUID REFERENCES products(id),
            tag_id UUID REFERENCES tags(id),
            PRIMARY KEY (product_id, tag_id)
        );

        CREATE TABLE IF NOT EXISTS notifications (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            vendor_id UUID NOT NULL REFERENCES vendors(id),
            product_id UUID NOT NULL REFERENCES products(id),
            message TEXT NOT NULL,
            is_read BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
} 