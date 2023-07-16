use serde::Deserialize;
use csv::ReaderBuilder;
use chrono::{NaiveDate, Duration, Datelike};
use chrono::offset::Local;
use std::cmp;
use std::fmt;
use std::collections::HashMap;
use rand::Rng;
// use rand::seq::SliceRandom;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;


#[derive(Debug, Clone)]
pub struct Schedule {
    pub tasks: Vec<Task>
}

#[derive(Debug, Deserialize, Clone)]
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
    #[serde(deserialize_with = "deserialize_plan_date")]
    pub start_date: NaiveDate,
    #[serde(rename = "Deadline")]
    #[serde(deserialize_with = "deserialize_plan_date")]
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
            "TASK: {}, Job No:{}-{}, Workshop:{}, CTR:{}, Deadline: {:?}, Est End: {}, deadline days: {},  Order: {}",
            self.id, self.job_number, self.card_number, self.workshop, self.ctr, self.deadline,self.est.unwrap_or_default(),self.deadline_days.unwrap_or(0),self.order.unwrap_or(0)) }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for task in &self.tasks {
            write!(f, "{}", task)?;
        }
        Ok(())
    }
}


fn deserialize_plan_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // println!("deserializing date");
    let date_str = String::deserialize(deserializer)?;
    match date_str.len() {
        0 => return Ok(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
        _ => (),
    }
    let (_,date_str) = date_str.split_at(4);
    // println!("date_str: {}",date_str);
    NaiveDate::parse_from_str(&date_str, "%d/%m/%y")
        .map_err(serde::de::Error::custom)
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match date_str.len() {
        0 => return Ok(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
        _ => (),
    }
    NaiveDate::parse_from_str(&date_str, "%d/%m/%y")
        .map_err(serde::de::Error::custom)
}

fn deserialize_long_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match date_str.len() {
        0 => return Ok(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
        _ => (),
    }
    NaiveDate::parse_from_str(&date_str, "%d/%m/%Y")
        .map_err(serde::de::Error::custom)
}
fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = date_str {
        NaiveDate::parse_from_str(&s, "%d/%m/%y").map(Some).map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}

fn deserialize_ctr<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // println!("deserializing ctr");
    let str = String::deserialize(deserializer)?;
    // println!("ctr: {}",str);
    let ctr: f64 = str.strip_suffix(" hrs").unwrap_or("0").parse().unwrap();
        // .map_err(serde::de::Error::custom)
    // let ctr = cmp::max(1,(ctr /8.) as u64);
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
    let workshops = ["ELEC", "FAB", "HYD", "MECH" , "M/C", "NDT", "P&M" , "PAINT"];
    let filtered_tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|task| workshops.contains(&task.workshop.as_str()))
        .collect();
    let mut schedule = Schedule { tasks: filtered_tasks };
    let today: NaiveDate = Local::now().date_naive();
    // println!("RAW Schedule: \n{}", schedule);
    let mut end_date = 0;
    let mut end_dates = vec![0;8];
    let mut workshop_id;
    for i in 0..schedule.tasks.len() {
        let ctr_days = cmp::max(1,schedule.tasks[i].ctr/8 as u64);
        let mut rng = rand::thread_rng();
        let workshop = schedule.tasks[i].workshop.as_str();
        match workshop {
            "ELEC" => workshop_id = 0,
            "FAB" => workshop_id = 1,
            "HYD" => workshop_id = 2,
            "MECH" => workshop_id = 3,
            "M/C" => workshop_id = 4,
            "NDT" => workshop_id = 5,
            "P&M" => workshop_id = 6,
            "PAINT" => workshop_id = 7,
            _ => panic!("Invalid Workshop found in inport csv file"),
        }
        // let  ctr_days = rng.gen_range(1..5);
        schedule.tasks[i].ctr = ctr_days;
        end_dates[workshop_id] += ctr_days;
        end_date = end_dates[workshop_id];
        // end_date += ctr_days;
        schedule.tasks[i].deadline = today.add_working_days(end_date as i64);
        schedule.tasks[i].deadline_days = Some(end_date);
    }  
    println!("Imported Job List: {:?} Jobs", schedule.tasks.len());
    println!("Processed Schedule: \n{} \n \nProcessing Complete", schedule);
    
    Ok(schedule)
}


// Generate a random schedule
pub fn create_schedule(jobs: usize) -> Schedule{
    let mut rng = rand::thread_rng();
    let workshops = ["ELEC", "FAB", "HYD", "MECH" , "M/C", "NDT", "P&M" , "PAINT"];
    let mut schedule = Schedule {
        tasks: Vec::new(),
    };
    // let mut start_date = 0;
    
    let today: NaiveDate = Local::now().date_naive();
    let mut end_date;
    let mut end_dates = vec![0;8];
    for i in 0..jobs{
        let  ctr = rng.gen_range(1..5);
        let workshop_id = 0;// rng.gen_range(0..8);
        let workshop_id =0;
        let workshop = workshops[workshop_id].to_string();
        end_dates[workshop_id] += ctr;
        end_date = end_dates[workshop_id];
        println!("Workshop: [{}] {}",workshop_id,workshop);
        println!("End Date: {:?}",end_dates);
        println!("End Date: {}",end_date);       
        let task = Task {
            id: i as i32,
            job_number: i.to_string(), //String::from("JB{}"+i as &str),// start: 0,
            card_number: String::from("6"),
            task: String::from("Task"),
            client: String::from("Client"),
            start_date: today,
            workshop,
            resources: 1,
            ctr,
            deadline: today.add_working_days(end_date as i64),
            deadline_days: Some(end_date),
            days: Option::None,
            order: Option::None,
            est: Option::None,
            // end: 0,
            
        };
        println!("Task: {}",task);

        schedule.tasks.push(task);
        // schedule.tasks.shuffle(&mut rng);
        // schedule.tasks.reverse()

    }
    // schedule.tasks.reverse();
    end_date = 0;
    // for i in 0..jobs {
    //     end_date += schedule.tasks[i].ctr ;
    //     schedule.tasks[i].deadline = today.add_working_days(end_date as i64);
    //     schedule.tasks[i].deadline_days = Some(end_date);
    // }

    // println!("Schedule Created: {}",schedule);
    // panic!("DEV PANIC: SCHEDULE CREATED");

    assert_eq!(schedule.tasks.len(), jobs);
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
    pub fn reorder_tasks_by_deadline_days(&mut self) {
        self.tasks.sort_by(|a, b| {
            let order_a = a.deadline_days.unwrap_or(std::u64::MAX);
            let order_b = b.deadline_days.unwrap_or(std::u64::MAX);
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
        let mut end_dates = vec![0;8];
        let mut workshop_id;
        let today: NaiveDate = Local::now().date_naive();
        for i in 0..dna.len() {
            let ctr = self.tasks[dna[i]].ctr;
            match self.tasks[dna[i]].workshop.as_str() {
                "ELEC" => workshop_id = 0,
                "FAB" => workshop_id = 1,
                "HYD" => workshop_id = 2,
                "MECH" => workshop_id = 3,
                "M/C" => workshop_id = 4,
                "NDT" => workshop_id = 5,
                "P&M" => workshop_id = 6,
                "PAINT" => workshop_id = 7,
                _ => workshop_id = 0,
            }
            end_dates[workshop_id] += ctr;
            end_date = end_dates[workshop_id];
            // end_date += ctr;
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

pub struct Workforce {
    pub resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Resource {
    #[serde(rename = "Sparrows ID")]
    pub sparrows_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Skills")]
    pub skill: String,
    #[serde(rename = "Level")]
    pub level: String,
    #[serde(rename = "Start Date")]
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(rename = "End Date")]
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "Workshop")]
    pub workshop: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Availability {
    #[serde(rename = "Payroll No.")]
    pub sparrows_id: String,
    #[serde(rename = "Job Name")]
    pub name: String,
    #[serde(rename = "Date")]
    #[serde(deserialize_with = "deserialize_long_date")]
    pub date: NaiveDate,
    #[serde(rename = "Hours")]
    pub available_hrs: u32,
}


pub fn import_resources_from_csv(file_path: &str) -> Result<Workforce, Box<dyn std::error::Error>> {
    let mut csv_reader = ReaderBuilder::new().from_path(file_path)?;
    let records: Result<Vec<Resource>, csv::Error> = csv_reader.deserialize().collect();
    let resources = records?;
    let workforce = Workforce { resources };
    Ok(workforce)

}

pub fn import_resource_calendar_from_csv(file_path: &str) -> Result<Vec<Availability>, Box<dyn std::error::Error>> {
    let mut csv_reader = ReaderBuilder::new().from_path(file_path)?;
    let records: Result<Vec<Availability>, csv::Error> = csv_reader.deserialize().collect();
    Ok(records.unwrap())

}

pub fn resource_count(workforce: Workforce)  {
    let mut workshop_counts: HashMap<String, usize> = HashMap::new();

    for resource in &workforce.resources {
        let count = workshop_counts.entry(resource.workshop.clone()).or_insert(0);
        *count += 1;
    }

    for (workshop, count) in workshop_counts {
        println!("{}: {} people", workshop, count);
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Resource: id: {}, Name: {}, Role: {}, Skill: {}, Level: {}, Start Date: {:?}, End Date: {:?} ",
            self.sparrows_id, self.name, self.role, self.skill, self.level, self.start_date, self.end_date
           )
         }
}

impl fmt::Display for Workforce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for resource in &self.resources {
            write!(f, "{}", resource)?;
        }
        Ok(())
    }
}

impl fmt::Display for Availability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resource: id: {}, Name: {}, Date: {:?}, Availability: {} ",
            self.sparrows_id, self.name, self.date, self.available_hrs
           )
         }
}
