#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use self::models::Post;
use self::schema::posts;
use self::schema::posts::dsl::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;



#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(data) => {
            println!("{:?}", data);
            HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable not found");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("No se pudo contruir la Pool");


    HttpServer::new(move|| {
        App::new().service(index).app_data(web::Data::new(pool.clone()))
    }).bind(("0.0.0.0", 9900))?.run().await

}
