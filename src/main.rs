#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rust_embed::RustEmbed;
use rust_embed_rocket::*;

#[derive(RustEmbed)]
#[folder = "frontend/build"]
struct Asset;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            Server::from_config(
                Asset,
                Config {
                    rank: 0,
                    serve_index: true,
                },
            )
        )
        .mount("/api", routes![index]).launch();
}
