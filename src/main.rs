mod routes;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}