use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use diesel::RunQueryDsl;
use crate::{module::Category, schema::category};

use crate::module::CategoryInsert;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/category")]
async fn category_insert(pool : web::Data<DbPool>,req_body : web::Json<CategoryInsert>) -> impl Responder {
    let data = req_body.into_inner();
    let mut  conn = pool.get().expect("could not connect");
    diesel::insert_into(category::table).values(&data).execute(&mut conn).expect("Failed to update !");
    HttpResponse::Ok().body("data Inserted !")
}

#[get("/category")]
async fn category_all(pool : web::Data<DbPool>) -> impl Responder {
    use self::category::dsl::*;
    let mut  conn = pool.get().expect("could Not connect ");
    let data:Vec<_> = category.load::<Category>(&mut conn).expect("could not get data ");
    HttpResponse::Ok().json(data)
}
