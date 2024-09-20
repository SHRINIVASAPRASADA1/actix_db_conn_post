use actix_web::{get, post, put, web::{self}, App, HttpResponse, HttpServer, Responder};
mod datastruct;
use datastruct::{products, Products, ProductsAdd};
mod tables;
use diesel::{query_dsl::methods::{FilterDsl, LimitDsl}, r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, RunQueryDsl};
use tables::establish_connection;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[get("/")]
async fn hello(pool: web::Data<DbPool>) -> impl Responder {
    use self::products::dsl::*;
    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> =  pool.get().expect("failed to connect");
    let prod: Vec<_> = products.limit(10).load::<Products>(&mut conn).expect("Failed to Readd");
    HttpResponse::Ok().json(prod)
}




#[post("/products")]
async fn echo(req_body: web::Json<ProductsAdd>, pool: web::Data<DbPool>) -> impl Responder {
    let data = req_body.into_inner();
    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> = pool.get().expect("Failed to get a connection from the pool");

    diesel::insert_into(products::table)
        .values(&data)
        .execute(&mut conn)
        .expect("Failed to insert data into the database");

    HttpResponse::Ok().body("Data Insert Done")
}



#[put["/products/{id}"]]
async fn update_product(req_body: web::Json<ProductsAdd>,pool: web::Data<DbPool>,id:web::Path<i32>) -> impl Responder {
    let product_id = id.into_inner();
    let data = req_body.into_inner();
    let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>> = pool.get().expect("Failed to get a connection from the pool");
    diesel::update(products::table.filter(products::id.eq(product_id)))  
        .set(&data)  
        .execute(&mut conn)
        .expect("Failed to update data in the database");
    HttpResponse::Ok().body("Data update Done")
    
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  
            .service(hello)
            .service(echo).service(update_product)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
