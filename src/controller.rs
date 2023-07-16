// use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use serde_json;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::thread;
#[derive(Serialize)]
pub struct JobOrder {
    job_order: Vec<u32>,
}


// use crate::models::*;

#[get("/")]
pub fn index() -> String {
    "Hello, world!".to_string()
}
#[get("/gaapi/schedule")]
pub fn schedule() -> Json<JobOrder>{
    thread::sleep(Duration::from_secs(6));
    let numbers: Vec<u32> = (0..=345 as u32).collect();
    let my_struct = JobOrder { job_order: numbers };
        let json = serde_json::to_string(&my_struct).expect("Failed to convert to JSON");
   Json(my_struct) 
}
#[get("/data")]
pub fn data() -> Json<JobOrder>{

    let numbers: Vec<u32> = (0..=345 as u32).collect();
    let my_struct = JobOrder { job_order: numbers };
        let json = serde_json::to_string(&my_struct).expect("Failed to convert to JSON");
   Json(my_struct) 
}
// #[get("/jobcards")]
// pub fn job_cards() -> Json<Vec<JobCard2>> {
//     use crate::schema::job_cards::dsl::job_cards;
//     let connection = &mut database::establish_connection();
//     job_cards.load::<JobCard2>(connection).map(Json).expect("Error loading jobcards")
// }
//
// #[get("/jobs")]
// pub fn jobs() -> Json<Vec<JobCard2>> {
//     use crate::schema::JobCard::dsl::JobCard;
//     let connection = &mut database::establish_connection();
//     JobCard.load::<JobCard2>(connection).map(Json).expect("Error loading jobcards")
// }
//
