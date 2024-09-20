use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::products;
use crate::schema::category;
use crate::schema::users;

use serde_json::Value;


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



#[derive(AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = users)]
pub struct UserInsert {
   pub username : String,
   pub email : String,
   pub contact : String,
   pub profile_image : String,
   pub password : String
}


