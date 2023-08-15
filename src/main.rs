use rocket::serde::json::{Value, Json};
use rocket_db_pools::{Database, mongodb};
#[macro_use] extern crate rocket;

mod routes;
use routes::users;

#[derive(Database)]
#[database("mongodb")]
pub struct MongoConn(mongodb::Client);

#[post("/mirror", data = "<data>", format = "json")]
fn mirror(data: Json<Value>) -> Json<Value> {
    data
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(MongoConn::init())
    .mount("/", routes![mirror])
    .mount("/users", routes![users::get_users,users::add_user, users::get_user])
}