use std::time::Instant;
use rand::{Rng, seq::SliceRandom};
use std::cmp::Ordering;
use std::fmt;
use version_check::Version;


// #[derive(Debug)]
struct Population {
    members: Vec<Dna>,
    evolution: Vec<u64>,
    search_space: Vec<f32>,
    dominance: Vec<usize>,
    
}

#[derive(Debug)]
struct Dna {
    genome_sequence: Vec<usize>,
    sex: Sex,
    fitness: u64,
    early: u64,
    late: u64
}

#[derive(Debug)]
enum Sex {
    Male,
    Female
}

impl Population {

    fn new(size: usize, genome_length: usize) -> Population {
        let mut population = Vec::with_capacity(size);
        let evolution = Vec::with_capacity(3000);
        // evolution.push(0);
        let search_spce = vec![0.0;genome_length * genome_length];
        let dominance = vec![0;genome_length];
        for _ in 0..size{
            population.push(Dna::new(genome_length)); 
        }
        Population { members: population , evolution, search_space: search_spce, dominance } 
    }
    
}

impl Dna {
    fn new(size: usize) -> Dna{
        let mut rng = rand::thread_rng();
        let mut person = Vec::with_capacity(size);
        for _ in 0..size {
            person.push(rng.gen_range(0..size));
        }
        let mut person = Vec::from_iter(0..size);
        person.shuffle(&mut rng);
        match rng.gen_range(0..2){
            0 => Dna { genome_sequence: person, sex: Sex::Male, fitness: 0, early:0, late:0},
            _ => Dna { genome_sequence: person, sex: Sex::Female, fitness: 0, early:0, late:0} 
        }
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "DNA: {}, Fitness:{}, Early {}, Late{}, Chromosme Length:{} Chromosome: {:?}",
            self.sex, self.fitness, self.early, self.late, self.genome_sequence.len(), self.genome_sequence
        );
    }
}

impl Ord for Dna {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.fitness).cmp(&(other.fitness))
    }
}

impl PartialOrd for Dna{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Dna {
    fn eq(&self, other: &Self) -> bool {
        (self.fitness) == (other.fitness)
    }
}

impl Eq for Dna { }

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Sex::Male => return write!(f,"Sex: Male  "),
            Sex::Female=> return write!(f,"Sex: Female")
         }

    }
}

impl Fitness for Population{
    fn calculate_fitness(&mut self, schedule: &Schedule) {
        let genome = self.members[0].genome_sequence.len();
        match genome{
            yes if yes == schedule.tasks.len() => {
            
            for i in 0..self.members.len(){
                // println!("Cecking fitness of member {i}");
                self.members[i].calculate_fitness(&schedule);
                // println!("Cecking fitness of member {i} = {} ", self.member[i].fitness);
            }

                // println!("Gene Checking: Passed")
            },
            _           => panic!("MISMACTHED GENOME SEQUENCES")
        }

    }
}

impl Breed for Population{
    fn breed(&mut self,top: usize) {
        // println!("Started breeding!!");
        // println!("First organism Selected == {}",self.members[0]);
        self.evolution.push(self.members[0].fitness);
        let mut rng = rand::thread_rng();
        let genome_length = self.members[0].genome_sequence.len();
        let mut next_generation = Population{members:Vec::with_capacity(self.members.len()), evolution: self.evolution.clone(),
         search_space: vec![0.0;genome_length*genome_length], dominance: vec![0;genome_length]};
        // let mut mutation_location = self.evolution.len()/100;
        let upper = top;
        for i in 0..upper {
            let parent1 = &self.members[i];
            for _ in 0..self.members.len()/upper{
                // let selected_organism = rng.gen_range(upper..self.members.len());
                // let parent2 = &self.members[selected_organism];


                let mut child = Dna::new(parent1.genome_sequence.len());
                child.genome_sequence = parent1.genome_sequence.clone();

                // crossover
                // let crossover_point = rng.gen_range(1..10);
                // for k in crossover_point..10{
                //     child.genome_sequence[k] = parent2.genome_sequence[k];
                // }
                // 
                // // // Second crossover_point
                // let crossover_point = rng.gen_range(crossover_point..10);
                // for k in crossover_point..10{
                //     child.genome_sequence[k] = parent2.genome_sequence[k];
                // }
                // // // Third crossover_point
                // let crossover_point = rng.gen_range(crossover_point..10);
                // for k in crossover_point..10{
                //     child.genome_sequence[k] = parent2.genome_sequence[k];
                // }
              

                // // Mutation
                // let mutations = rng.gen_range(0..9);
                // for _ in 0..mutations{
                //     let mutationed_gene = rng.gen_range(0..genome_length);
                //     child.genome_sequence[mutationed_gene] = rng.gen_range(0..genome_length);
                // }


                // if rng.gen_range(0..9) == 0{
                //     let mutationed_gene = rng.gen_range(0..genome_length);
                //     child.genome_sequence[mutationed_gene] = rng.gen_range(0..genome_length);
                // }

                let mutationed_gene = rng.gen_range(0..genome_length);
                child.genome_sequence[mutationed_gene] = rng.gen_range(0..genome_length);

               



                // Implement fitness check masking out one genone to find if in the correct position
                // let mut dominance = vec![0;genome_length];
                // child.calculate_fitness(&ideal, &schedule);
                // let before = child.fitness;
                // for i in 0..genome_length{
                //     let tmp = child.genome_sequence[i];
                //     child.genome_sequence[i] = 0;
                //     child.calculate_fitness(&ideal, &schedule);
                //     let after = child.fitness;
                //     
                //
                //     child.genome_sequence[i] = tmp;
                //     if after != before{
                //         dominance[i] = tmp;
                //         self.dominance[i] = tmp;
                //     }
                //     if self.dominance[i] == 0{
                //         child.genome_sequence[i] = tmp;
                //     } else {
                //         child.genome_sequence[i] = self.dominance[i];
                //     }
                // }
                //
                // let mutationed_gene = rng.gen_range(0..genome_length);
                // let mut choices = Vec::with_capacity(genome_length);
                // for i in 0..genome_length{
                //     if dominance[i] == 0{
                //         choices.push(i);
                //     }
                // }
                //
                // child.genome_sequence[mutationed_gene] = choices[rng.gen_range(0..choices.len())];
                // if choices.len() == 1{
                //     child.genome_sequence[mutationed_gene] = choices[0];
                //     child.genome_sequence[0] = mutationed_gene;
                // }






                // if  before <= 0{
                //     
                //     println!("Child: {:?}",child);
                //     println!("Dominance: {:?}",dominance);
                //     panic!();
                // }

                
                // child.genome_sequence[mutation_location] = rng.gen_range(mutation_location..genome_length);
                // mutation_location += 1;
                // if mutation_location == genome_length{
                //     mutation_location = 0;
                // }
                // println!("Mutation Location: {}",mutation_location);
                // 

                // println!("Parent 1: {:?}",parent1.genome_sequence);
                // println!("Parent 2: {:?}",parent2.genome_sequence);
                // println!("Child 1:  {:?}",child.genome_sequence);
 

                // Repair Dna
                let mut ideal = Vec::from_iter(0..genome_length);
                let mut replace = Vec::<usize>::with_capacity(genome_length);


                // println!("Ideal {:?}",ideal);
                // println!("DNA: {:?}",child.genome_sequence);
                // println!("{}",genome_length);
                for i in 0..genome_length{
                    match ideal.contains(&child.genome_sequence[i]) {
                    // match child.genome_sequence.contains(&ideal[i]) {
                        true => {
                            // println!("{}::found:{}",i,child.genome_sequence[i]);
                            let index = ideal.iter().position(|&r| r == child.genome_sequence[i]).unwrap();
                            ideal.remove(index);
                        }
                        false => {
                            replace.push(i);
                            // println!("{}::error:{}",i,child.genome_sequence[i]);
                        }
                        
                    }
                }

                ideal.shuffle(&mut rng);
                for i in 0..replace.len(){
                    let idx  = replace.pop().unwrap();
                    child.genome_sequence[idx] = ideal[i];
                }
                next_generation.members.push(child);
           }
        }
        
        // copy search space into next generation
        for i in 0..genome_length*genome_length {
            next_generation.search_space[i] = self.search_space[i];
        }
        
        // Add the search of the previous generation to the search space
        for i in 0..self.members.len(){
            for j in 0..self.members[i].genome_sequence.len(){
                let search_loc = j * genome_length + self.members[i].genome_sequence[j];
                let prev = self.search_space[search_loc] ;
                next_generation.search_space[search_loc] =  prev + 2.55;
            }
        }

        // Keep the best organism from the previous generation in the new generation 
        let top = 20;
        let top = self.members.len()/(100/top);
        for _ in 0..top{
            next_generation.members.pop();
        }
            for i in 0..top{
            let mut last_best = Dna::new(genome_length);
            last_best.genome_sequence = self.members[i].genome_sequence.clone();
            next_generation.members.push(last_best);
        }
        *self = next_generation;
    }

}

trait Breed {
    fn breed(&mut self,top:usize);
}

trait Fitness {
    fn calculate_fitness(&mut self, schedule: &Schedule);
}




// trait Display {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result; 
// }











fn info () {
    println!();
    println!("--------------------------------"); 
    println!("Genetic Algorithm Coded in Rust");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!(""); 
    println!("Genetic Algorithm Version: {}", VERSION);
    // println!("Rust Version"); 
    match Version::read() {
        Some(d) => println!("Rust Version: {}", d),
        None => println!("Failed to read the version no. of Rust.")
    };

    // println!("???"); 
    println!(""); 
    println!("--------------------------------"); 
    println!("Genetic Algoritm Version: {}", VERSION); 
    println!("--------------------------------"); 
    println!(""); 
}






#[derive(Debug)]
struct Schedule {
    tasks: Vec<Task>
}
#[derive(Debug)]
struct Task {
    // id: usize,
    // start: u64,
    // end: u64,
    deadline: u64,
    ctr: u64,
}

// Generate a random schedule
fn create_schedule(jobs: usize) -> Schedule{
    let mut rng = rand::thread_rng();
    let mut schedule = Schedule {
        tasks: Vec::new(),
    };
    // let mut start_date = Utc::now(); // Local::now().na:wxffive_local();
    let mut start_date = 0; 
    for _ in 0..jobs{
        let  ctr = rng.gen_range(1..5);
        // let ctr = 5;
        start_date = start_date + ctr;
        let task = Task {
            // id: i,
            // start: 0,
            ctr,
            deadline: start_date,
            // end: 0,
            
        };
    schedule.tasks.push(task);
    // schedule.tasks.shuffle(&mut rng);
    // schedule.tasks.reverse();
    }
    // schedule.tasks.reverse();
    schedule
 }


fn simulation(pop_size: usize, genome_length: usize, schedule: &Schedule) -> Population{


    info();

    let start = Instant::now();
    let mut world = Population::new(pop_size, genome_length);
    let duration = start.elapsed();
    let first = true;
    println!("Time elapsed to generate population of {} x {} is: {:?}", pop_size, genome_length, duration);
    world.calculate_fitness(&schedule);
    world.members.sort();
 
    let mut i = 0;
    loop {


        i=i+1;
        world.breed(20);
        world.calculate_fitness(&schedule);
        world.members.sort();
        // world.evolution.push(world.members[0].fitness);
        let mut cover = 0;
        for i in 0..world.search_space.len(){
            if world.search_space[i] != 0. {
                cover +=1;
            }
        }
        let pcover = 100.*cover as f32/world.search_space.len() as f32;
        if first == true {
        print!("{}: Mini Fitness {}, Early: {} :: Late {}, Size: {:?} x {}  Coverage: {:.2}%    \r",i,world.members[0].fitness, world.members[0].early, world.members[0].late, world.members[0].genome_sequence.len(), world.members.len(), pcover);
        }
        // Optimal solution found
        if world.members[0].fitness ==0 && first == true{
            println!();
            println!("Solution found in generation {}",i);
            // println!("{:?}",world.members[0]);
            println!("{}: Min Fitness {}, Early: {} :: Late {}, Size: {:?} x {}  Coverage: {:.2}%    \r",i,world.members[0].fitness, world.members[0].early, world.members[0].late, world.members[0].genome_sequence.len(), world.members.len(), pcover);
            let duration = start.elapsed();
            println!("Simulation completed in {:?}", duration);            
            // println!("Dominance: {:?}",world.dominance);
            // first = false;
            return world;
        }
    }
}






// User implemented Trait for fitness of DNA 
impl Fitness for Dna {
    fn calculate_fitness(&mut self, schedule: &Schedule) {
        // let ideal = vec![0,1,2,3,4,5,6,7,8,9];
        let mut score = 0;
        let mut early = 0;
        let mut late = 0;
        let mut start_date = 0;

        // Test for vector of estimated end dates (as days from start)
        for i in 0..self.genome_sequence.len(){
            if let Some(task) = schedule.tasks.get(self.genome_sequence[i]) {
                // println!("Genome: {}", self.genome_sequence[i]);
                start_date = start_date + task.ctr;        

                // println!("Task: {} Start: {} Est.: {} Deadline: {} CTR: {}", task.id, task.start, start_date, task.deadline, task.ctr);
                if task.deadline < start_date {
                    // score +=1;
                    // late +=1;
                    late = late + (start_date - task.deadline);
                    score = score + (start_date - task.deadline);
                    // println!("Late");
                }
                if task.deadline > start_date {
                    // score +=1;
                    // early +=1;
                    score = score + (task.deadline - start_date);
                    early = early + (task.deadline - start_date);
                    // println!("Early");
                }
                } else {
                    println!("Missing");
                    panic!("Task not found");
                }
        }

        self.fitness = score;
        self.early = early;
        self.late = late;
    }
}

fn main (){

    let pop_size:usize = 50;
    let genome_length = 200;   // 363
    let schedule = create_schedule(genome_length);

    let otimal = simulation(pop_size, genome_length, &schedule);
    println!("Optimal Solution: {:?}", otimal.members[0]);
    println!("Optimal Solution found in generation {}", otimal.evolution.len());
    println!("Dominance: {:?}",otimal.dominance);
}
