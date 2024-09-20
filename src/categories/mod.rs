use actix_web::{get, post, put, web, HttpResponse, Responder};
use diesel::{query_dsl::methods::FilterDsl, r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection};
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

#[put("/category/{ids}")]
async fn category_update(pool:web::Data<DbPool>,req_body : web::Json<CategoryInsert>,ids:web::Path<i32>) -> impl Responder {
    let category_id = ids.into_inner();
    let data = req_body.into_inner();
     let mut conn = pool.get().expect("failed to connect to database ");
    let _ = diesel::update(category::table.filter(category::id.eq(category_id))).set(&data).execute(&mut conn).expect("Update Failed");
    HttpResponse::Ok().body("Updated !")
}

#[get("/category/{ids}")]
async fn category_one(pool:web::Data<DbPool>,ids:web::Path<i32>) -> impl Responder {
    use self::category::dsl::*;
    let category_id: i32 = ids.into_inner();
    let mut conn = pool.get().expect("could not initiate connection");
    let data = category.filter(id.eq(category_id)).load::<Category>(&mut conn).expect("Failed to Readd");
    HttpResponse::Ok().json(data)
}