use std::str::FromStr;
use rocket::futures::StreamExt;
use rocket::serde::json::{Value, Json, json};
use serde::{Serialize, Deserialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::bson::doc;
use crate::MongoConn;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    _id: ObjectId,
    name: String,
    age: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AppState {
    users: Vec<User>,
}

#[post("/getUsers")]
pub async fn get_users(mongo_conn: &MongoConn) -> Value {
    let users:Vec<User> = mongo_conn.database("Rocket").collection::<User>("Users")
    .find(None, None)
    .await
    .unwrap()
    .collect::<Vec<Result<User, _>>>()
    .await
    .into_iter()
    .map(|cursor| cursor.unwrap())
    .collect();
    println!("{:?}", users);
    json!(users)
}

#[post("/getUser", data = "<data>", format = "json")]
pub async fn get_user(data: Json<Value>, mongo_conn: &MongoConn) -> Value {
    let user_id: &str = data.get("user_id").unwrap().as_str().unwrap();
    let user: User = mongo_conn.database("Rocket").collection::<User>("Users")
    .find_one(doc! {"_id": ObjectId::from_str(user_id).unwrap()}, None)
    .await
    .unwrap()
    .ok_or("User not found")
    .unwrap();
    println!("{:?}", user);
    json!(user)
}

#[post("/addUser", data = "<user>", format = "json")]
pub async fn add_user(user: Json<User>, mongo_conn: &MongoConn) -> String {
    let user_id = ObjectId::new();
    let user = User {
        _id: user_id,
        name: user.name.clone(),
        age: user.age,
    };
    mongo_conn.database("Rocket").collection::<User>("Users")
    .insert_one(user, None).await.unwrap();
    user_id.to_string()
}