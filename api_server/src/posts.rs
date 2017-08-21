use diesel;
use diesel::prelude::*;
use diesel::associations::HasTable;
use DbConn;

use models::*;
use schema::posts::dsl::*;

use rocket_contrib::Json;

#[get("/")]
fn index(conn: DbConn) -> QueryResult<Json<Vec<Post>>> {
    posts
        .filter(published.eq(true))
        .limit(10)
        .load::<Post>(&*conn)
        .map(|ps| Json(ps))
}

#[post("/", data = "<post>")]
fn create(conn: DbConn, post: Json<NewPost>) -> QueryResult<Json<Post>> {
    let post = post.0;

    diesel::insert(&post)
        .into(posts::table())
        .get_result::<Post>(&*conn)
        .map(|p| Json(p))
}

#[post("/<pid>/publish")]
fn publish(conn: DbConn, pid: i32) -> QueryResult<Json<Post>> {
    diesel::update(posts.find(pid))
        .set(published.eq(true))
        .get_result::<Post>(&*conn)
        .map(|p| Json(p))
}

#[delete("/<pid>")]
fn delete(conn: DbConn, pid: i32) -> () {
    diesel::delete(posts.filter(id.eq(pid)))
        .execute(&*conn)
        .expect("Error deleting posts");
}