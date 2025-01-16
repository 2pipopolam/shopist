use axum::{
    extract::{State, Path, Form},
    response::Html,
};
use sea_orm::{
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    QueryOrder,
    ColumnTrait,
    ActiveModelTrait,
    Set,
};
use crate::{
    error::AppError,
    models::{item, shop},
};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "shop_items.html")]
struct ShopItemsTemplate {
    shop: shop::Model,
    items: Vec<item::Model>,
}

// Template for partial items list update
#[derive(Template)]
#[template(path = "items_list.html")]
struct ItemsListTemplate {
    shop: shop::Model,
    items: Vec<item::Model>,
}

#[derive(Deserialize)]
pub struct UpdateQuantityForm {
    pub value: i32,
}

pub async fn get_shop_items(
    State(ref db): State<DatabaseConnection>,
    Path(shop_id): Path<i32>,
) -> Result<Html<String>, AppError> {
    let shop = shop::Entity::find_by_id(shop_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;
    let items = item::Entity::find()
        .filter(item::Column::ShopId.eq(shop_id))
        .order_by_asc(item::Column::Name)
        .all(db)
        .await?;

    let template = ShopItemsTemplate { shop, items };
    Ok(Html(template.render().map_err(|e| AppError::BadRequest(e.to_string()))?))
}

pub async fn create_item(
    State(ref db): State<DatabaseConnection>,
    Path(shop_id): Path<i32>,
    Form(new_item): Form<item::CreateItem>,
) -> Result<Html<String>, AppError> {
    let shop = shop::Entity::find_by_id(shop_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let _item = item::ActiveModel {
        name: Set(new_item.name),
        quantity: Set(new_item.quantity),
        is_picked: Set(false),
        shop_id: Set(shop_id),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let items = item::Entity::find()
        .filter(item::Column::ShopId.eq(shop_id))
        .order_by_asc(item::Column::Name)
        .all(db)
        .await?;

    let template = ItemsListTemplate { shop, items };
    Ok(Html(template.render().map_err(|e| AppError::BadRequest(e.to_string()))?))
}

pub async fn toggle_item(
    State(ref db): State<DatabaseConnection>,
    Path((shop_id, item_id)): Path<(i32, i32)>,
) -> Result<Html<String>, AppError> {
    let shop = shop::Entity::find_by_id(shop_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let item = item::Entity::find_by_id(item_id)
        .filter(item::Column::ShopId.eq(shop_id))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut item: item::ActiveModel = item.into();
    item.is_picked = Set(!item.is_picked.unwrap());
    let _updated_item = item.update(db).await?;

    let items = item::Entity::find()
        .filter(item::Column::ShopId.eq(shop_id))
        .order_by_asc(item::Column::Name)
        .all(db)
        .await?;

    let template = ItemsListTemplate { shop, items };
    Ok(Html(template.render().map_err(|e| AppError::BadRequest(e.to_string()))?))
}

pub async fn update_quantity(
    State(ref db): State<DatabaseConnection>,
    Path((shop_id, item_id)): Path<(i32, i32)>,
    Form(form): Form<UpdateQuantityForm>,
) -> Result<Html<String>, AppError> {
    let shop = shop::Entity::find_by_id(shop_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let item = item::Entity::find_by_id(item_id)
        .filter(item::Column::ShopId.eq(shop_id))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut item: item::ActiveModel = item.into();
    item.quantity = Set(form.value);
    let _updated_item = item.update(db).await?;

    let items = item::Entity::find()
        .filter(item::Column::ShopId.eq(shop_id))
        .order_by_asc(item::Column::Name)
        .all(db)
        .await?;

    let template = ItemsListTemplate { shop, items };
    Ok(Html(template.render().map_err(|e| AppError::BadRequest(e.to_string()))?))
}
