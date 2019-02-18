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
fn index(_db_conn: RustyDbConn) -> &'static str {
    // Rocket uses the RustyDbConn request guard to provide us with a database
    // connection from a managed pool.
    "Hello, from Rust! (with a database connection!)"
}

#[database("rustydb")]
pub struct RustyDbConn(PgConnection);

fn main() {
    rocket::ignite()
        .attach(RustyDbConn::fairing())
        .mount("/api", routes![index])
        .launch();
}
