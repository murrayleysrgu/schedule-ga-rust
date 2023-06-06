use serde::Deserialize;
use csv::ReaderBuilder;
use chrono::{NaiveDate, Duration, Datelike};
use chrono::offset::Local;
use std::fmt;
use rand::{Rng, seq::SliceRandom};
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;


#[derive(Debug)]
pub struct Schedule {
    pub tasks: Vec<Task>
}

#[derive(Debug, Deserialize)]
pub struct Task {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Job Number")]
    pub job_number: String,
    #[serde(rename = "Card No.")]
    pub card_number: String,
    #[serde(rename = "Task Name")]
    pub task: String,
    #[serde(rename = "Client")]
    pub client: String,
    #[serde(rename = "Start")]
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(rename = "Deadline")]
    #[serde(deserialize_with = "deserialize_date")]
    pub deadline: NaiveDate,
    pub deadline_days: Option<u64>,
    #[serde(rename = "Workshop")]
    pub workshop: String,
    #[serde(rename = "Resoureses")]
    pub resources: i32,
    #[serde(rename = "CTR Hours")]
    #[serde(deserialize_with = "deserialize_ctr")]
    pub ctr: u64,
    pub days: Option<f32>,
    pub order: Option<i32>,
    pub est: Option<NaiveDate>,
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "TASK: {}, Job No:{}-{}, Task:{}, CTR:{}, Deadline: {:?}, Est End: {}, deadline days: {},  Order: {}",
            self.id, self.job_number, self.card_number, self.task, self.ctr, self.deadline,self.est.unwrap_or_default(),self.deadline_days.unwrap_or(0),self.order.unwrap_or(0)) }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for task in &self.tasks {
            write!(f, "{}", task)?;
        }
        Ok(())
    }
}


fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    println!("deserializing date");
    let date_str = String::deserialize(deserializer)?;
    match date_str.len() {
        0 => return Ok(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
        _ => (),
    }
    let (_,date_str) = date_str.split_at(4);
    println!("date_str: {}",date_str);
    NaiveDate::parse_from_str(&date_str, "%d/%m/%Y")
        .map_err(serde::de::Error::custom)
}

fn deserialize_ctr<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    println!("deserializing ctr");
    let str = String::deserialize(deserializer)?;
    println!("ctr: {}",str);
    let ctr: f64 = str.strip_suffix(" hrs").unwrap_or("1").parse().unwrap();
        // .map_err(serde::de::Error::custom)
    Ok(ctr as u64) 
}

// fn is_weekday(date: &NaiveDate) -> bool {
//     match date.weekday() {
//         Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => true,
//         _ => false,
//     }
// }

pub trait WorkingDays {
     fn add_working_days(&self, days: i64) -> NaiveDate;
}


impl WorkingDays for NaiveDate {
    fn add_working_days(&self, days: i64) -> NaiveDate {
        let mut date = *self;
        let mut days_left = days;
        while days_left > 0 {
            date = date + Duration::days(1);
            if date.weekday() != chrono::Weekday::Sat && date.weekday() != chrono::Weekday::Sun {
                days_left -= 1;
            }
        }
        date
    }



   
}
    fn working_days_until(start:NaiveDate, other: NaiveDate) -> i64 {
        let mut date = start;
        let mut days = 0;
        while date < other {
            date = date + Duration::days(1);
            if date.weekday() != chrono::Weekday::Sat && date.weekday() != chrono::Weekday::Sun {
                days += 1;
            }
        }
        days
    }

pub fn read_schedule_from_csv(file_path: &str) -> Result<Schedule, Box<dyn std::error::Error>> {
    let mut csv_reader = ReaderBuilder::new().from_path(file_path)?;
    let records: Result<Vec<Task>, csv::Error> = csv_reader.deserialize().collect();
    let tasks = records?;
    // let records = records.unwrap();
    // let mut end_date = 0;
    // for i in 0..records.len() {
    //     end_date += records[i].ctr ;
    //     records[i].deadline_days = Some(end_date);
    //     // records[i].order = Some(i as i32);
    //     // records[i].est = Some(begin.unwrap().add_working_days(end_date as i64));
    //     // records[i].est = Some(today.add_working_days(end_date as i64));
    // }

    let mut schedule = Schedule { tasks };
    let today: NaiveDate = Local::now().date_naive();
    println!("RAW Schedule: \n{}", schedule);
    let mut end_date = 0;
    for i in 0..schedule.tasks.len() {
        end_date += schedule.tasks[i].ctr/8 as u64; 
        schedule.tasks[i].deadline = today.add_working_days(end_date as i64);
        schedule.tasks[i].deadline_days = Some(end_date);
    }   
    Ok(schedule)
}


// Generate a random schedule
pub fn create_schedule(jobs: usize) -> Schedule{
    let mut rng = rand::thread_rng();
    let mut schedule = Schedule {
        tasks: Vec::new(),
    };
    // let mut start_date = 0;
    
    let today: NaiveDate = Local::now().date_naive();
    let mut end_date = 0;
    for i in 0..jobs{
        let  ctr = rng.gen_range(1..5);
        end_date = end_date + ctr;
        let task = Task {
            id: i as i32,
            job_number: i.to_string(), //String::from("JB{}"+i as &str),// start: 0,
            card_number: String::from("6"),
            task: String::from("Task"),
            client: String::from("Client"),
            start_date: today,
            workshop: String::from("Workshop"),
            resources: 1,
            ctr,
            deadline: today.add_working_days(end_date as i64),
            deadline_days: Some(end_date),
            days: Option::None,
            order: Option::None,
            est: Option::None,
            // end: 0,
            
        };
        schedule.tasks.push(task);
        schedule.tasks.shuffle(&mut rng);
        // schedule.tasks.reverse()

    }
    schedule.tasks.reverse();
    end_date = 0;
    for i in 0..jobs {
        end_date += schedule.tasks[i].ctr ;
        schedule.tasks[i].deadline = today.add_working_days(end_date as i64);
        schedule.tasks[i].deadline_days = Some(end_date);
    }

    println!("Schedule Created:_+_+_+_+ {:?}",schedule);
    schedule
 }



impl Schedule {
    pub fn reorder_tasks_by_order(&mut self) {
        self.tasks.sort_by(|a, b| {
            let order_a = a.order.unwrap_or(std::i32::MAX);
            let order_b = b.order.unwrap_or(std::i32::MAX);
            order_a.cmp(&order_b)
        });
    }
    pub fn reorder_tasks_by_id(&mut self) {
        self.tasks.sort_by(|a, b| {
            let order_a = a.id;
            let order_b = b.id;
            order_a.cmp(&order_b)
        });
    }
    pub fn reorder_tasks_by_deadline(&mut self) {
        self.tasks.sort_by(|a, b| {
            let order_a = a.deadline;
            let order_b = b.deadline;
            order_a.cmp(&order_b)
        });
    }
    pub fn reorder_tasks_by_est(&mut self) {
        self.tasks.sort_by(|a, b| {
            let order_a = a.est.unwrap_or(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap());
            let order_b = b.est.unwrap_or(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap());
            order_a.cmp(&order_b)
        });
    }
    pub fn update_schedule(&mut self) {
        let today: NaiveDate = Local::now().date_naive();
        for i in 0..self.tasks.len() {
            let end_date = working_days_until(self.tasks[i].deadline , today) as u64;
            // end_date += self.tasks[i].ctr ;
            self.tasks[i].deadline_days = Some(end_date);
       }
    }

    pub fn update_schedule_est(&mut self, dna: &Vec<usize>) {
        let mut end_date = 0;
        let today: NaiveDate = Local::now().date_naive();
        for i in 0..dna.len() {
            let ctr = self.tasks[dna[i]].ctr;
            end_date += ctr;
            self.tasks[dna[i]].est = Some(today.add_working_days(end_date as i64));
            self.tasks[dna[i]].order = Some(i as i32);
            // println!("{} {} - {} + {}",self.tasks[dna[i]].id,self.tasks[dna[i]].est.unwrap(),self.tasks[dna[i]].ctr, end_date);
       }
    }


    pub fn export_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut writer = WriterBuilder::new().from_writer(file);

        // Write the CSV header
        writer.write_record(&[
            "ID",
            "Job Number",
            "Card Number",
            "Task",
            "Client",
            "Start Date",
            "Deadline",
            "Deadline Days",
            "Workshop",
            "Resources",
            "CTR",
            "Days",
            "Order",
            "Est",
        ])?;

        // Write each task to the CSV file
        for task in &self.tasks {
            let start_date_str = task.start_date.to_string();
            let deadline_str = task.deadline.to_string();
            let deadline_days_str = task.deadline_days.map(|days| days.to_string()).unwrap_or_default();
            let resources_str = task.resources.to_string();
            let ctr_str = task.ctr.to_string();
            let days_str = task.days.map(|days| days.to_string()).unwrap_or_default();
            let order_str = task.order.map(|order| order.to_string()).unwrap_or_default();
            let est_str = task.est.map(|est| est.to_string()).unwrap_or_default();

            writer.write_record(&[
                task.id.to_string(),
                task.job_number.clone(),
                task.card_number.clone(),
                task.task.clone(),
                task.client.clone(),
                start_date_str,
                deadline_str,
                deadline_days_str,
                task.workshop.clone(),
                resources_str,
                ctr_str,
                days_str,
                order_str,
                est_str,
            ])?;
        }

        writer.flush()?;
        Ok(())
    }
}
