pub mod categories;
pub mod notifications;
pub mod products;
pub mod tags;

use actix_web::{HttpResponse, Responder};

pub use categories::*;
pub use notifications::*;
pub use products::*;
pub use tags::*;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Backend is running!")
}
