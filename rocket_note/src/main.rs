#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::{Json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes {
    pub address: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/mine")]
fn mine() -> Json<Value> {
    Json(json!({
        "message": "new block forged",
        "index": 1,
        "transactions": [],
        "proof": "proof",
        "previous_hash": "hash",
    }))
}

#[get("/chain")]
fn chain() -> Json<Value> {
    Json(json!({
        "chain": "chain",
        "length": 3,
    }))
}

#[get("/nodes/resolve")]
fn nodes_resolve() -> Json<Value> {
    Json(json!({
        "message": "message",
        "chain": "chain",
    }))
}

#[post("/nodes/register", format = "application/json", data = "<nodes>")]
fn nodes_register(nodes: Json<Nodes>) -> Json<Value> {
    Json(json!({
        "message": "New nodes have been added",
        "total_nodes": 3,
    }))
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
fn transactions(transaction: Json<Transaction>) -> Json<Value> {
    Json(json!({
        "message": "new transaction created, index 3",
    }))
}


fn main() {
    rocket::ignite()
        .mount("/", 
            routes![hello, 
                mine, 
                chain,
                nodes_resolve,
                nodes_register,
                transactions,
            ]
        )
        .launch();
}
