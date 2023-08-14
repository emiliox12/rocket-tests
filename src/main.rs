use rocket::futures::StreamExt;
use rocket::serde::json::{Value, Json, json};
use rocket::State;
use rocket_db_pools::mongodb::bson::Document;
use serde::{Serialize, Deserialize};
use rocket_db_pools::{Database, mongodb};
use std::sync::Mutex;
use mongodb::bson::oid::ObjectId;

#[macro_use] extern crate rocket;

#[derive(Database)]
#[database("mongodb")]
struct MongoConn(mongodb::Client);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    #[serde(default)]
    id: ObjectId,
    name: String,
    age: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AppState {
    users: Vec<User>,
}

#[get("/users")]
async fn get_users(state_mut: &State<Mutex<AppState>>, mongo_conn: &MongoConn) -> Value {
    let users_cursor = mongo_conn.database("Rocket").collection::<Document>("Users").find(None, None).await;
    for result in users_cursor {
        println!("title: {}", result);
    }
    json!(state_mut.lock().unwrap().users.clone())
}

#[post("/users", data = "<user>", format = "json")]
fn add_user(user: Json<User>, state_mut: &State<Mutex<AppState>>) -> String {
    let mut state: std::sync::MutexGuard<'_, AppState> = state_mut.lock().unwrap();
    let user_id = state.users.len() as i32 + 1;
    state.users.push(User {
        id: ObjectId::new(),
        name: user.name.to_string(),
        age: user.age,
    });
    user_id.to_string()
}

#[post("/mirror", data = "<data>", format = "json")]
fn mirror(data: Json<Value>) -> Json<Value> {
    data
}

#[launch]
fn rocket() -> _ {
    let state = Mutex::new(AppState {
        users: Vec::new(),
    });
    rocket::build()
    .manage(state)
    .attach(MongoConn::init())
    .mount("/", routes![get_users,add_user, mirror])
}