#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod schema;

use rocket_contrib::{Json, Value};

mod user;
use user::{User};

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: db::Connection) -> Json<User> {
    let user = user.into_inner();

    let insert = User {
        id: None,
        name: user.name,
        identity: user.identity,
        hometown: user.hometown
    };

    Json(User::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(User::read(&connection)))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, connection: db::Connection) -> Json<Value> {
    let user = user.into_inner();

    let update = User {
        id: Some(id),
        name: user.name,
        identity: user.identity,
        hometown: user.hometown
    };

    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "status": User::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/users", routes![create, read, update, delete])
        .launch();
}