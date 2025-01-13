use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::*;

use crate::{error::AppError, models::item};
use crate::models::item::CreateItem;

pub async fn get_shop_items(
    State(ref db): State<DatabaseConnection>,
    Path(shop_id): Path<i32>,
) -> Result<Json<Vec<item::Model>>, AppError> {
    let items = item::Entity::find()
        .filter(item::Column::ShopId.eq(shop_id))
        .order_by_asc(item::Column::Name)
        .all(db)
        .await?;
    Ok(Json(items))
}

pub async fn create_item(
    State(ref db): State<DatabaseConnection>,
    Path(shop_id): Path<i32>,
    Json(new_item): Json<CreateItem>,
) -> Result<Json<item::Model>, AppError> {
    let item = item::ActiveModel {
        name: Set(new_item.name),
        quantity: Set(new_item.quantity),
        is_picked: Set(false),
        shop_id: Set(shop_id),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(Json(item))
}

pub async fn toggle_item(
    State(ref db): State<DatabaseConnection>,
    Path((shop_id, item_id)): Path<(i32, i32)>,
) -> Result<Json<item::Model>, AppError> {
    let item = item::Entity::find_by_id(item_id)
        .filter(item::Column::ShopId.eq(shop_id))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut item: item::ActiveModel = item.into();
    item.is_picked = Set(!item.is_picked.unwrap());
    let updated_item = item.update(db).await?;

    Ok(Json(updated_item))
}

pub async fn update_quantity(
    State(ref db): State<DatabaseConnection>,
    Path((shop_id, item_id)): Path<(i32, i32)>,
    Json(quantity): Json<i32>,
) -> Result<Json<item::Model>, AppError> {
    let item = item::Entity::find_by_id(item_id)
        .filter(item::Column::ShopId.eq(shop_id))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut item: item::ActiveModel = item.into();
    item.quantity = Set(quantity);
    let updated_item = item.update(db).await?;

    Ok(Json(updated_item))
}
