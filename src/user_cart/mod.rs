use crate::module::{User, UserCartInsert};
use crate::schema::users;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::{
    query_dsl::methods::FilterDsl,
    r2d2::{self, ConnectionManager, PooledConnection},
    ExpressionMethods, PgConnection, RunQueryDsl,
};

type Dbpol = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/add_to_cart")]
async fn UserCart(
    pool: web::Data<Dbpol>,
    req_body: web::Json<UserCartInsert>,
    session: Session,
) -> impl Responder {
    let data = req_body.into_inner();
    let conn = pool.get().expect("connection failed");
    let user = session.get::<String>("user").expect("could not get ");

    println!("user is {:?}", user);

    println!("{:?}", data);

    HttpResponse::Ok().body("ok")
}

async fn check_user_exist(pool: web::Data<Dbpol>, account: String, password: String) -> bool {
    let res = web::block(move || {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> =
            pool.get().expect("connection failed");

         users::table
            .filter(users::username.eq(account)) 
            .filter(users::password.eq(password))
            .load::<User>(&mut conn)
    }).await;


    match res {
        Ok(users) => {
            print!("");
            true
        }
        Err(err) => {
            println!("The problem is: {:?}", err.to_string());
            false
        }
    }
}
