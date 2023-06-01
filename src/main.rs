use schedule_ga_rust::{create_schedule, simulation};

fn main (){

    let pop_size:usize = 50;
    let genome_length = 500;   // 363
    let schedule = create_schedule(genome_length);

    let otimal = simulation(pop_size, genome_length, &schedule);
    println!("Optimal Solution: {:?}", otimal.members[0]);
    println!("Optimal Solution found in generation {}", otimal.evolution.len());
    println!("Dominance: {:?}",otimal.dominance);
}
