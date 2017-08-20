#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate lazy_static;
extern crate dotenv;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use dotenv::dotenv;
use std::env;

mod pg_pool;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    // initialize env
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", routes![index]).launch();
}
