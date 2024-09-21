use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::*;
use serde_json::Value;
use chrono::{NaiveDate};

#[derive(Serialize,Deserialize,Queryable)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub keywords: Value,
}

#[derive(Insertable,Deserialize,AsChangeset,Debug)]
#[diesel(table_name = category)]
pub struct CategoryInsert {
    pub title: String,
    pub subtitle: String,
    pub keywords: Value,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Products {
    pub id: i32,
    pub title: String,
    pub category_id: i32,
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
    pub tag:Value,      
    pub images: Value,  
    pub features: Value, 
}


#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = products)]
pub struct ProductsAdd {
    pub title: String,
    pub category_id: i32,
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
    pub tag: Value,
    pub images: Value,
    pub features: Value,
}



#[derive(Queryable,Serialize,Deserialize)]
pub struct User {
   pub id : i32,
   pub username : String,
   pub email : String,
   pub contact : String,
   pub profile_image : String,
   pub password : String
}



#[derive(Insertable,AsChangeset,Serialize,Deserialize)]
#[diesel(table_name = users)]
pub struct UserInsert {
   pub username : String,
   pub email : String,
   pub contact : String,
   pub profile_image : String,
   pub password : String
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserAddress {
    pub id: i32,
    pub user_id: i32,
    pub pin_code: i32, 
    pub area: Option<String>, // Mark as Option to match SQL definition
    pub city: Option<String>, // Mark as Option
    pub contact: Option<String>, // Mark as Option
    pub added_date: NaiveDate, // Use NaiveDate to match SQL
    pub other: Option<String>, // Mark as Option
}

#[derive(Insertable, Deserialize,AsChangeset, Debug)]
#[diesel(table_name = user_address)]
pub struct UserAddressInsert {
    pub user_id: i32,
    pub pin_code: i32,
    pub area: Option<String>, // Mark as Option
    pub city: Option<String>, // Mark as Option
    pub contact: Option<String>, // Mark as Option
    pub added_date: NaiveDate, // Change to NaiveDate
    pub other: Option<String>, // Mark as Option
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserCart {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>, // Mark as Option
    pub added_date: NaiveDate, // Use NaiveDate
    pub is_active: Option<bool>,
    pub extra: Value, // Mark as Option
}

#[derive(Insertable, Deserialize,AsChangeset, Debug)]
#[diesel(table_name = user_cart)]
pub struct UserCartInsert {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>, // Mark as Option
    pub added_date: NaiveDate, // Change to NaiveDate
    pub is_active: Option<bool>,
    pub extra: Value, // Mark as Option
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserOrder {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>, // Mark as Option
    pub added_date: NaiveDate, // Use NaiveDate
    pub is_accepted: Option<bool>,
    pub is_cancelled: Option<bool>,
    pub is_online_pay: bool, // Keep as is since it's non-nullable
    pub extra: Value, // Mark as Option
}

#[derive(Insertable, Deserialize,AsChangeset, Debug)]
#[diesel(table_name = user_order)]
pub struct UserOrderInsert {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>, // Mark as Option
    pub added_date: NaiveDate, // Change to NaiveDate
    pub is_accepted: Option<bool>,
    pub is_cancelled: Option<bool>,
    pub is_online_pay: bool, // Keep as is
    pub extra: Value, // Mark as Option
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct OrderStatus {
    pub id: i32,
    pub product_id: i32,
    pub order_id: i32,
    pub last_updated: NaiveDate, // Use NaiveDate
    pub is_accepted: Option<bool>,
    pub locations: Value, // Mark as Option
}

#[derive(Insertable, Deserialize,AsChangeset, Debug)]
#[diesel(table_name = order_status)]
pub struct OrderStatusInsert {
    pub product_id: i32,
    pub order_id: i32,
    pub last_updated: NaiveDate, // Change to NaiveDate
    pub is_accepted: Option<bool>,
    pub locations: Value, // Mark as Option
}
