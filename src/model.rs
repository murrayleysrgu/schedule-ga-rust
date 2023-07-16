use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono;
use crate::schema::bird_sighting;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct JobCard {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub job_name: String,
    pub job_number: String,
    pub job_workshop: String,
    pub job_status: String,
    pub due_date: chrono::NaiveDateTime,
    pub job_contact: String,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct JobCard2 {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub job_name: String,
    pub job_number: String,
    pub job_workshop: String,
    pub job_status: String,
    pub due_date: chrono::NaiveDateTime,
    pub job_contact: String,
}


