use diesel::{prelude::*, sql_types::Double};
use diesel::sql_types::{Integer, Jsonb};
use serde::{Deserialize, Serialize};
use diesel_json::Json;
use serde_json::Number;  // Import Diesel's JSON type

table! {
   products (id) {
       id -> Integer,
       title -> Text,
       price -> Integer,
       description -> Text,
       discount -> Integer,
       quantity -> Integer,
       category -> Text,
       brand -> Text,
       thumbnail -> Text,
       weight -> Float,
       height -> Float,
       is_active -> Bool,
       tag -> Jsonb,
       images -> Jsonb,
       features -> Jsonb,
   }
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Products {
    pub id: i32,
    pub title: String,
    pub price: i32,
    pub description: String,
    pub discount: i32,
    pub quantity: i32,
    pub category: String,
    pub brand: String,
    pub thumbnail: String,
    pub weight: f32,
    pub height: f32,
    pub is_active: bool,
    pub tag: Json<serde_json::Value>,     // Use diesel_json::Json
    pub images: Json<serde_json::Value>,  // Use diesel_json::Json
    pub features: Json<serde_json::Value>, // Use diesel_json::Json
}

#[derive(Insertable, Deserialize, Debug,AsChangeset)]
#[diesel(table_name = products)]
pub struct ProductsAdd {
    pub title: String,
    pub price: i32,
    pub description: String,
    pub discount: i32,
    pub quantity: i32,
    pub category: String,
    pub brand: String,
    pub thumbnail: String,
    pub weight: f32,
    pub height: f32,
    pub is_active: bool,
    pub tag: Json<serde_json::Value>,     // Use diesel_json::Json
    pub images: Json<serde_json::Value>,  // Use diesel_json::Json
    pub features: Json<serde_json::Value>, // Use diesel_json::Json
}
