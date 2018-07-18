#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[allow(unused)]
#[macro_use]
extern crate rocket_contrib;

#[allow(unused)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate dotenv;
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;

mod pg_pool;
pub use pg_pool::DbConn;

mod schema;
// mod models;

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust!"
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", routes![index])
        .launch();
}
