use axum::{
    extract::{State, Form},
    response::Html,
};
use sea_orm::{
    DatabaseConnection,
    EntityTrait,
    QueryOrder,
    Set,
    ActiveModelTrait
};
use askama::Template;

use crate::{error::AppError, models::shop};

#[derive(Template)]
#[template(path = "shops.html")]
struct ShopsTemplate {
    shops: Vec<shop::Model>,
}

#[derive(Template)]
#[template(path = "shops_list.html")]
struct ShopsListTemplate {
    shops: Vec<shop::Model>,
}

pub async fn index(
    State(ref db): State<DatabaseConnection>,
) -> Result<Html<String>, AppError> {
    let shops = shop::Entity::find()
        .order_by_asc(shop::Column::Name)
        .all(db)
        .await?;

    let template = ShopsTemplate { shops };
    template.render().map_err(|e| AppError::BadRequest(e.to_string())).map(Html)
}

pub async fn create_shop(
    State(ref db): State<DatabaseConnection>,
    Form(new_shop): Form<shop::CreateShop>,
) -> Result<Html<String>, AppError> {
    let _shop = shop::ActiveModel {
        name: Set(new_shop.name),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let shops = shop::Entity::find()
        .order_by_asc(shop::Column::Name)
        .all(db)
        .await?;
    
    let template = ShopsListTemplate { shops };
    Ok(Html(template.render().map_err(|e| AppError::BadRequest(e.to_string()))?))
}
