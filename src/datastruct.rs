
use diesel::{prelude::{Insertable, Queryable}, table};
use serde::{Deserialize,Serialize};

table! {
   products (id) {
       id -> Integer,
       title -> Text,
       price -> Integer,
       description -> Text,
   }
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct Products{
   pub id :i32,
   pub title : String,
   pub price : i32,
   pub description : String
}


#[derive(Insertable,Deserialize,Debug)]
#[diesel(table_name = products)]
pub struct ProductsAdd{
   pub title : String,
   pub price : i32,
   pub description : String
}

