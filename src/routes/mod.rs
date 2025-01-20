use axum::{
    Router,
    routing::{get, put, post, delete},
    response::Redirect,
};
use sea_orm::DatabaseConnection;
use tower_http::services::ServeDir;
use crate::handlers::{shops, items};
use axum::extract::Path;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(shops::index))
        .route("/shops", post(shops::create_shop))
        .route("/shops/:id", get(|Path(id): Path<i32>| async move {
            Redirect::to(&format!("/shops/{}/items", id))
        }))
        .route(
            "/shops/:id/items",
            get(items::get_shop_items)
            .post(items::create_item)
            .delete(items::delete_all_items)
        )
        .route("/shops/:shop_id/items/:item_id/toggle", 
            put(items::toggle_item))
        .route(
            "/shops/:shop_id/items/:item_id/quantity",
            put(items::update_quantity))
        .route("/shops/:shop_id/items/:item_id",
            delete(items::delete_item))
        .nest_service(
            "/static",
            ServeDir::new("static")
        )
        .with_state(db)
}
