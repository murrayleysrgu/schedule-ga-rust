use schedule_ga_rust::{
    create_schedule,
    genetics, 
    read_schedule_from_csv,
    GeneticAlgorithmConfig, CrossoverMethod, MutationMethod, OrganismType
};
use schedule_ga_rust::schedule::Schedule;
// use rand::seq::SliceRandom;
// use rand::thread_rng;

mod schedule;
use schedule::{import_resources_from_csv, import_resource_calendar_from_csv,resource_count};


use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::Writer;

fn main ()-> Result<(), Box<dyn Error>>{

    let pop_size:usize = 66;
    let genome_length = 300;   // 363
    // let csv = true;
    let csv = false;
    let visualise = true;
    // let visualise = false;
    let diagnostics = false;
    let mut schedule: Schedule;

    if csv == false {
        println!("Creating Schedule from imported csv");
       schedule = create_schedule(genome_length);
    } else {
        println!("Creating Random Schedule");
       // schedule = read_schedule_from_csv("book1.csv").unwrap();
       schedule = read_schedule_from_csv("plan_export.csv").unwrap();
       println!("Job List Imported");
    }

    // schedule.reorder_tasks_by_deadline_days();


    let resources = import_resources_from_csv("resources.csv"); 
    match resources {
        Ok(r) => {
            resource_count(r);
            println!("Resources Imported")
        },
        Err(e) => println!("Error: {}", e),
    };

    let calendar = import_resource_calendar_from_csv("PersonDailyActivityDetail.csv");

    // let resources = import_resources_from_csv("resources.csv").unwrap(); 
    // println!("Resources: {}", resources);
    // panic!("manual stop to check code");
    // println!("Schedule:+ \n{}", schedule);
    // let mut rng = thread_rng();
    // schedule.tasks.shuffle(&mut rng);
    // schedule.reorder_tasks_by_id();
    // println!("Schedule:++ \n{}", schedule);
    println!("First Task: {}", schedule.tasks[0]);
    let config = GeneticAlgorithmConfig {
        population_size: pop_size,
        mutation_rate: 0.0,
        max_generations: 100000,
        // organism_type: OrganismType::Diploid,
        organism_type: OrganismType::Haploid,
        crossover_method: CrossoverMethod::Epigenetic,
        mutation_method: MutationMethod::Uniform,
        visualise,
        diagnostics,
    };

    let optimal = genetics(&schedule, config);

    // let optimal = simulation(schedule, config);
    // let optimal = visual_simulation(schedule, config);
    // println!("\nOptimal Solution: {}", optimal.unwrap().members[0]);
    // println!("Optimal Solution found in generation {}", optimal.unwrap().evolution.len());
    // println!("{:?}",optimal.unwrap().evolution);

let mut writer = Writer::from_path("epi-evolution.csv")?;

        for value in optimal.unwrap().evolution {
            writer.write_record(&[value.to_string()])?;
        }


    // println!("Dominance: {:?}",optimal.unwrap().dominance);

    // let opt_solution = &optimal.unwrap().members[0].genome_sequence;
    //
    // schedule.update_schedule_est(opt_solution);
    // schedule.reorder_tasks_by_order();
    // // println!("\nOptimal Schedule:\n{}", schedule);
    // schedule.export_to_csv("optimal.csv")
    //     .expect("Faile to export optimal schedule to csv");

    // println!("member: {}", optimal.members[0]);

Ok(())
}
