use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::*;

use crate::{error::AppError, models::shop};
use crate::models::shop::CreateShop;

pub async fn get_shops(
    State(ref db): State<DatabaseConnection>,
) -> Result<Json<Vec<shop::Model>>, AppError> {
    let shops = shop::Entity::find()
        .order_by_asc(shop::Column::Name)
        .all(db)
        .await?;
    Ok(Json(shops))
}

pub async fn create_shop(
    State(ref db): State<DatabaseConnection>,
    Json(new_shop): Json<CreateShop>,
) -> Result<Json<shop::Model>, AppError> {
    let shop = shop::ActiveModel {
        name: Set(new_shop.name),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(Json(shop))
}

pub async fn get_shop(
    State(ref db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<shop::Model>, AppError> {
    let shop = shop::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(shop))
}
