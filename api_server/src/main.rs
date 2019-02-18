#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

// #[macro_use]
extern crate diesel;

mod schema;
// mod models;

use rocket_contrib::databases::diesel::PgConnection;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust!"
}

#[database("rustydb")]
pub struct RustyDbConn(PgConnection);

fn main() {
    rocket::ignite()
        .attach(RustyDbConn::fairing())
        .mount("/api", routes![index])
        .launch();
}
