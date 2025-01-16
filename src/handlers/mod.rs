pub mod items;
pub mod shops;

use crate::models::{shop, item};
use askama::Template;

#[derive(Template)]
#[template(path = "shops.html")]  
pub struct ShopsTemplate {
    pub shops: Vec<shop::Model>,
}

#[derive(Template)]
#[template(path = "shop_items.html")]  
pub struct ShopItemsTemplate {
    pub shop: shop::Model,
    pub items: Vec<item::Model>,
}
