mod categories;
mod module;
mod product;
mod schema;

use crate::categories::*;
use crate::product::*;

use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
mod tables;
use tables::establish_connection;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(products_get)
            .service(products_insert)
            .service(products_update)
            .service(product_one)
            .service(category_all)
            .service(category_insert)
            .service(category_one)
            .service(category_update).service(category_product)
            .route("/ping", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
