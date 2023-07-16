#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
mod models;
mod schema;
mod controller;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        controller::index,
        controller::schedule,
        controller::data,
        controller::job_cards,
        controller::jobs
    ])
}
