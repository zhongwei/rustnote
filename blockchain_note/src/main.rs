#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate chrono;
extern crate url;
extern crate reqwest;
extern crate open;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashSet;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use chrono::prelude::*;
use rocket::response::content;
use rocket_contrib::{Json, Value};
use url::Url;

lazy_static! {
    static ref GLOBAL_BLOCKCHAIN: Mutex<Blockchain> = Mutex::new(Blockchain::new());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: i64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    proof: i64,
    previous_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Nodes {
    address: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Blockchain {
    desc: String,
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: HashSet<String>,
}

#[derive(Deserialize, Debug)]
struct ChainResponse {
    length: i64,
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            desc: "🔗This is my private chain!🍸🐕🚀".to_string(),
            chain: Vec::new(),
            current_transactions: Vec::new(),
            nodes: HashSet::new(),
        };

        blockchain.new_block(100, String::from("1"));
        blockchain
    }

    fn new_transaction(&mut self, sender: String, recipient: String, amount: f64) -> i64 {
        self.current_transactions.push(Transaction {
            sender,
            recipient,
            amount,
        });

        self.chain.last().unwrap().index + 1
    }

    fn new_block(&mut self, proof: i64, previous_hash: String) -> Block {
        let utc: DateTime<Utc> = Utc::now();
        let phash = if !previous_hash.is_empty() {
            previous_hash
        } else {
            Blockchain::hash(&self.chain.last().unwrap())
        };

        let nblock = &Block {
            index: (self.chain.len() + 1) as i64,
            timestamp: utc.timestamp(),
            transactions: self.current_transactions.to_vec(),
            previous_hash: phash,
            proof,
        };

        self.current_transactions = Vec::new();
        self.chain.push(nblock.clone());
        nblock.clone()
    }

    fn hash(block: &Block) -> String {
        let mut hasher = Sha256::new();
        let block_string = serde_json::to_string(block).unwrap();
        hasher.input_str(&block_string);

        hasher.result_str()
    }

    fn proof_of_work(&self, last_proof: i64) -> i64 {
        let mut proof = 0i64;
        while Blockchain::valid_proof(last_proof, proof) == false {
            proof = proof + 1;
        }
        proof
    }

    fn valid_proof(last_proof: i64, proof: i64) -> bool {
        let mut hasher = Sha256::new();
        let guess = &format!("{}{}", last_proof, proof);
        hasher.input_str(guess);
        let output = hasher.result_str();
        &output[..4] == "0000"
    }

    fn register_nodes(&mut self, address: String) {
        let url = Url::parse(&address).unwrap();
        let host_port = format!("{}:{}", url.host_str().unwrap(), url.port().unwrap());
        self.nodes.insert(host_port);
    }

    fn resolve_conflicts(&mut self) -> bool {
        let mut max_length: i64 = self.chain.len() as i64;
        let mut new_chain: Vec<Block> = Vec::new();

        for node in self.nodes.iter() {
            let url = format!("http://{}/chian", node);
            let buf_content = get_content(&url).unwrap();
            let content: ChainResponse = serde_json::from_str(&buf_content).unwrap();

            if content.length > max_length && Blockchain::valid_chain(&content.chain) {
                max_length = content.length;
                new_chain = content.chain.clone();
            }  
        }

        if new_chain.len() > 0 {
            self.chain = new_chain.clone();
            return true;
        }
        false
    }

    fn valid_chain(chain: &Vec<Block>) -> bool {
        let mut last_block = chain.first().unwrap();
        let mut current_index = 1;

        while current_index < chain.len() {
            let block = &chain[current_index];
            println!("[Last block] {:?}", last_block);
            println!("[current block] {:?}", block);

            if block.previous_hash != Blockchain::hash(last_block) {
                return false;
            }

            if !Blockchain::valid_proof(last_block.proof, block.proof) {
                return false;
            }

            last_block = &block;
            current_index = current_index + 1;
        }

        true
    }
}

fn get_content(url: &str) -> reqwest::Result<String> {
    reqwest::get(url)?.text()
}

const INDEX_HTML: &'static [u8] = include_bytes!("../static/index.html");
const BULMA_CSS: &'static [u8] = include_bytes!("../static/vendor/bulma.min.css");

#[get("/")]
fn index() -> content::Html<String> {
    let index_html: String = String::from_utf8_lossy(INDEX_HTML).to_string();
    content::Html(index_html)
}

#[get("/vendor/bulma.min.css")]
fn bulma_css() -> content::Css<String> {
    let bulma_css: String = String::from_utf8_lossy(BULMA_CSS).to_string();
    content::Css(bulma_css)
}

#[get("/mine")]
fn mine() -> Json<Value> {
    let mut blockchain = GLOBAL_BLOCKCHAIN.lock().unwrap();
    let chain = blockchain.chain.to_vec();
    let last_block = chain.last().unwrap();
    let last_proof: i64 = last_block.proof;
    let proof = blockchain.proof_of_work(last_proof);

    blockchain.new_transaction(
        String::from("0"),
        String::from("57e430de001d498fbf6e493a79665d57"),
        1.0,
    );

    let block = blockchain.new_block(proof, String::new());

    Json(json!({
        "message": "new block forged",
        "index": block.index,
        "transactions": block.transactions,
        "proof": block.proof,
        "previous_hash": block.previous_hash,
    }))
}

#[get("/chain")]
fn chain() -> Json<Value> {
    let blockchain = GLOBAL_BLOCKCHAIN.lock().unwrap();

    Json(json!({
        "desc": blockchain.desc,
        "chain": blockchain.chain,
        "length": blockchain.chain.len(),
    }))
}

#[get("/nodes/resolve")]
fn nodes_resolve() -> Json<Value> {
    let mut blockchain = GLOBAL_BLOCKCHAIN.lock().unwrap();

    let _message = if blockchain.resolve_conflicts() {
        "Our chain was replaced"
    } else {
        "Our chain is authoritative"
    };

    Json(json!({
        "message": "message",
        "chain": "chain",
    }))
}

#[post("/nodes/register", format = "application/json", data = "<nodes>")]
fn nodes_register(nodes: Json<Nodes>) -> Json<Value> {
    if nodes.address.len() <= 0 {
        return Json(json!({
            "error": "send some address",
        }))
    }

    let mut blockchain = GLOBAL_BLOCKCHAIN.lock().unwrap();

    for node in nodes.address.iter() {
        blockchain.register_nodes(node.clone())
    }

    Json(json!({
        "message": "message",
        "total_nodes": 3,
    }))
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
fn transactions(transaction: Json<Transaction>) -> Json<Value> {
    let mut blockchain = GLOBAL_BLOCKCHAIN.lock().unwrap();
    let index = blockchain.new_transaction(
        transaction.sender.clone(),
        transaction.recipient.clone(),
        transaction.amount,
    );

    Json(json!({
        "message": format!("new transaction created, index {}", index),
    }))
}

fn main() {
    if open::that("http://localhost:8000").is_ok() {
        println!("Look at your browser !");
    };

    rocket::ignite()
        .mount("/",
            routes![
                index,
                bulma_css,
                mine,
                chain,
                nodes_resolve,
                nodes_register,
                transactions,
            ],
        )
        .launch();
}

#[test]
fn it_works() {
    let mut blockchain = Blockchain::new();
    blockchain.register_nodes(String::from("http://localhost:8000"));
    println!("nodes: {:?}", blockchain.nodes);
    println!("{:?}", blockchain.resolve_conflicts());
}