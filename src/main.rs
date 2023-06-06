use schedule_ga_rust::{create_schedule,simulation, read_schedule_from_csv, GeneticAlgorithmConfig, CrossoverMethod, MutationMethod};
use schedule_ga_rust::schedule::Schedule;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main (){

    let pop_size:usize = 50;
    let genome_length = 50;   // 363
    let csv = false;
    let mut schedule: Schedule;

    if csv == false {
       schedule = create_schedule(genome_length);
    } else {
       // schedule = read_schedule_from_csv("book1.csv").unwrap();
       schedule = read_schedule_from_csv("plan_export.csv").unwrap();
    }
    
    // println!("Schedule:+ \n{}", schedule);
    let mut rng = thread_rng();
    schedule.tasks.shuffle(&mut rng);
    schedule.reorder_tasks_by_id();
    println!("Schedule:++ \n{}", schedule);

    let config = GeneticAlgorithmConfig {
        population_size: pop_size,
        mutation_rate: 0.1,
        crossover_method: CrossoverMethod::Uniform,
        mutation_method: MutationMethod::Uniform,
    };

    let optimal = simulation(&schedule, config);
    println!("Optimal Solution: {:?}", optimal.members[0]);
    println!("Optimal Solution found in generation {}", optimal.evolution.len());
    println!("Dominance: {:?}",optimal.dominance);
    let opt_solution = &optimal.members[0].genome_sequence;

    schedule.update_schedule_est(opt_solution);
    schedule.reorder_tasks_by_order();
    println!("\nOptimal Schedule:\n{}", schedule);
    schedule.export_to_csv("optimal.csv")
        .expect("Failed to export optimal schedule to csv");

    let str = "23";
    let num: i32 = str.parse().unwrap();
    println!("{} -> {}",str,num);

    let str = "23.5 hrs";
    let num: f32 = str.strip_suffix(" hrs").unwrap().parse().unwrap();
    println!("{} -> {}",str,num as i32);
}
