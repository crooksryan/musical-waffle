
mod routes;
mod dataobj;
mod database;

#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};

use rocket_db_pools::Database;

use crate::routes::{
        user,
        index,
        hello,
        post
};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![hello, post, user])
        .mount("/assets", FileServer::from(relative!("/front-end/dist/assets")))
        .attach(Records::init())
        .mount("/db", routes![read])
        .launch()
        .await?;
    Ok(())
}
