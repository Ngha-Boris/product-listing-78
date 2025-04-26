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
    // Create vendors table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS vendors (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create categories table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create tags table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Insert default categories if they don't exist
    sqlx::query(
        r#"
        INSERT INTO categories (id, name)
        VALUES 
            ('11111111-1111-1111-1111-111111111111', 'Handcrafts'),
            ('22222222-2222-2222-2222-222222222222', 'Food & Drinks'),
            ('33333333-3333-3333-3333-333333333333', 'Clothing & Fashion'),
            ('44444444-4444-4444-4444-444444444444', 'Home & Decor'),
            ('55555555-5555-5555-5555-555555555555', 'Art & Collectibles'),
            ('66666666-6666-6666-6666-666666666666', 'Agriculture')
        ON CONFLICT (name) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    // Insert default tags if they don't exist
    sqlx::query(
        r#"
        INSERT INTO tags (id, name)
        VALUES 
            ('77777777-7777-7777-7777-777777777777', 'Handmade'),
            ('88888888-8888-8888-8888-888888888888', 'Organic'),
            ('99999999-9999-9999-9999-999999999999', 'Fair Trade'),
            ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'Traditional'),
            ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'Sustainable'),
            ('cccccccc-cccc-cccc-cccc-cccccccccccc', 'Eco-friendly'),
            ('dddddddd-dddd-dddd-dddd-dddddddddddd', 'Vegan'),
            ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', 'Natural')
        ON CONFLICT (name) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    // Create products table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            vendor_id UUID NOT NULL REFERENCES vendors(id),
            name VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            price FLOAT8 NOT NULL,
            image_url TEXT NOT NULL,
            is_draft BOOLEAN DEFAULT true,
            is_verified BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create product_categories table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_categories (
            product_id UUID REFERENCES products(id),
            category_id UUID REFERENCES categories(id),
            PRIMARY KEY (product_id, category_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create product_tags table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_tags (
            product_id UUID REFERENCES products(id),
            tag_id UUID REFERENCES tags(id),
            PRIMARY KEY (product_id, tag_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create notifications table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notifications (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            vendor_id UUID NOT NULL REFERENCES vendors(id),
            product_id UUID NOT NULL REFERENCES products(id),
            message TEXT NOT NULL,
            is_read BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
