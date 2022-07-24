#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url undefined");


    let con = PgConnection::establish(&db_url).expect("connection refused");
    
    use self::models::{Post, NewPost, PostSimplicado};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "Mi segundo post",
        body: "lorem ipsum",
        slug: "segundo-post"
    };
    let post: Post = diesel::insert_into(posts::table).values(&new_post).get_result(&con).expect("Error insierting data");

    //Select * from post
    println!("Query sin limites");
    let post_result = posts.load::<Post>(&con).expect("Error on query excecution");
    
    for post in post_result {
        println!("{}", post.title);
    }

    // println!("Query con limites");
    // let post_result = posts.limit(1).load::<Post>(&con).expect("Error on query excecution");

    // for post in post_result {
    //     println!("{}", post.title);
    // }

    // Query con columnas especificas
    let post_result = posts.select((title, body)).load::<PostSimplicado>(&con).expect("Error on query excecution");
    for post in post_result {
        println!("{:?}", post);
    }

}
