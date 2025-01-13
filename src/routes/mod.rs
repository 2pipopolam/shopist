use axum::{
    Router,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;

use crate::handlers::{shops, items};

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/shops", get(shops::get_shops).post(shops::create_shop))
        .route("/shops/:id", get(shops::get_shop))
        .route(
            "/shops/:id/items",
            get(items::get_shop_items).post(items::create_item),
        )
        .route("/shops/:shop_id/items/:item_id/toggle", put(items::toggle_item))
        .route(
            "/shops/:shop_id/items/:item_id/quantity",
            put(items::update_quantity),
        )
        .with_state(db)
}
