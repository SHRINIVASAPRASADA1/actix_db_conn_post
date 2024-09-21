use actix_session::Session;
use actix_web::{get, post, web::{self}, HttpResponse, Responder};
use diesel::{query_dsl::methods::FilterDsl, r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, RunQueryDsl};
use crate::{module::{User, UserInsert}, schema::users};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/user")]
async fn crete_user(pool : web::Data<DbPool>, req_body : web::Json<UserInsert>) -> impl Responder {
     let  data = req_body.into_inner();
     let mut conn = pool.get().expect("could not initiate connection ");
     diesel::insert_into(users::table).values(&data).execute(&mut conn).expect("Could not get db");
     HttpResponse::Ok().body("ok")
}

#[get("/user/{ids}")]
async fn get_user(pool:web::Data<DbPool>,ids : web::Path<i32>) -> impl Responder {
    let ids = ids.into_inner();
    use self::users::dsl::*;
    let mut conn = pool.get().expect("could not get db ");
    let client = users.filter(id.eq(ids)).load::<User>(&mut conn).expect("could not get") ;
    HttpResponse::Ok().json(client)
}

#[post("/user/login")]
async fn user_login(pool : web::Data<DbPool>,req_body : web::Json<User>,session : Session) -> impl Responder {
    let data = req_body.into_inner();
    println!("{:?}",data.password);
    println!("{:?}",data.username);
    println!("{:?}",data.email);
    println!("{:?}",data.contact);
    let _ = session.insert("user", data.email);
    HttpResponse::Ok().body("authenticated")
}