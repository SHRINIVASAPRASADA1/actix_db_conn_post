use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
extern crate dotenv;
use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
     r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
