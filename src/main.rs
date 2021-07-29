#[macro_use]
extern crate rocket;

use rand::prelude::*;
use rocket::{Build, build, Rocket};
use rocket::serde::json::Json;

mod core;

#[get("/")]
fn index() -> String {
    let mut rng = rand::thread_rng();
    format!("OK{:06}", rng.gen_range(0..0xffff))
}

#[get("/entities")]
fn list_entities() -> Json<Vec<core::Entity>> {
    Json(core::list_entities())
}

fn rocket() -> Rocket<Build> {
    build()
        .mount("/", routes![
            index,
            list_entities
        ])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
