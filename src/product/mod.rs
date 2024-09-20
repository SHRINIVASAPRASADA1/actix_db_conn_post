use actix_web::{get, post, put, web, HttpResponse, Responder};
use diesel::{r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use crate::module::{Products, ProductsAdd};

use crate::schema::products;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[get("/products")]
pub async fn products_get(pool: web::Data<DbPool>) -> impl Responder {
    use self::products::dsl::*;
    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("failed to connect");
        
    let prod: Vec<_> = products
        .limit(10)
        .load::<Products>(&mut conn)
        .expect("Failed to Readd");
    HttpResponse::Ok().json(prod)
}

#[post("/products")]
pub async fn products_insert(
    req_body: web::Json<ProductsAdd>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let data = req_body.into_inner();
    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    diesel::insert_into(products::table)
        .values(&data)
        .execute(&mut conn)
        .expect("Failed to insert data into the database");

    HttpResponse::Ok().body("Data Insert Done")
}

#[put["/products/{id}"]]
pub async fn products_update(
    req_body: web::Json<ProductsAdd>,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let product_id = id.into_inner();
    let data = req_body.into_inner();
    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");
    diesel::update(products::table.filter(products::id.eq(product_id)))
        .set(&data)
        .execute(&mut conn)
        .expect("Failed to update data in the database");
    HttpResponse::Ok().body("Data update Done")
}

#[get("/products/{ids}")]
pub async fn product_one(
    pool: web::Data<DbPool>,
    ids: web::Path<i32>,
) -> impl Responder {
    let product_id = ids.into_inner();
    use self::products::dsl::*;

    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("could not connect ");
    let prod: Vec<_> = products
        .filter(id.eq(product_id))
        .load::<Products>(&mut conn)
        .expect("Failed to Readd");
        println!("this function hit");
    HttpResponse::Ok().json(prod)
}
