#![feature(plugin)]
#![feature(proc_macro)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

use maud::{html, Markup};
use std::borrow::Cow;

#[get("/<name>")]
fn hello<'a>(name: Cow<'a, str>) -> Markup {
    html! {
        h1 { "Hello, " (name) "!" }
        p "Nice to meet you!"
        p {"🥁Hi, " (name) "!"}
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}