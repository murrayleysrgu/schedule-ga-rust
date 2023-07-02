use std::time::Instant;
use std::cmp::Ordering;
use std::fmt;
use rand::{Rng, seq::SliceRandom};
// use rand::distributions::Standard;
use version_check::Version;
use std::cmp;

pub mod schedule;
pub use schedule::{Schedule, Task, WorkingDays, create_schedule, read_schedule_from_csv};


use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
// use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// #[derive(Debug)]
pub struct Population {
    pub members: Vec<Dna>,
    pub evolution: Vec<u64>,
    pub search_space: Vec<f32>,
    pub current_search_space: Vec<f32>,
    pub dominance: Vec<usize>,
    
}

#[derive(Debug)]
pub struct Dna {
    pub genome_sequence: Vec<usize>,
    pub female_chromosome: Vec<usize>,
    pub male_chromosome: Vec<usize>,
    pub m_chromosome: Vec<usize>,
    pub epigenome: Vec<i32>,
    pub sex: Sex,
    pub fitness: u64,
    pub errors: u64,
    pub early: u64,
    pub late: u64
}

#[derive(Debug)]
pub enum Sex {
    Male,
    Female
}
 


impl Population {

    fn new(size: usize, genome_length: usize) -> Population {
        let mut population = Vec::with_capacity(size);
        let evolution = Vec::with_capacity(3000);
        // evolution.push(0);
        let search_spce = vec![0.0;genome_length * genome_length];
        let current_search_space = vec![0.0;genome_length * genome_length];
        let dominance = vec![0;genome_length];
        for _ in 0..size{
            population.push(Dna::new(genome_length)); 
        }
        Population { members: population , evolution, search_space: search_spce, current_search_space: current_search_space,dominance } 
    }
    pub fn sort_members_by_fitness(&mut self) {
        self.members.sort_by(|a, b| a.fitness.cmp(&b.fitness));
    }
    pub fn find_minimum_fitness(&self) -> Option<u64> {
        self.members.iter().map(|dna| dna.fitness).min()
    }

    pub    fn draw(&self, frame: &mut [u8],zoom: usize, config: &GeneticAlgorithmConfig, visualisation: &Visualisation) {
        
 
        let genome_length = self.members[0].genome_sequence.len();
        let pop_size = self.members.len();
        let width = genome_length*zoom;
        let zone = width * pop_size ;

        // println!("Size {}",frame.len());
        // let win_width = frame.len() / 4 / (zoom * pop_size*3 + genome_length) ;
        // println!("zoom {}",zoom);
        // println!("win_width {}",win_width);
        // panic!("stop");

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        // if visualisation  == Visualisation::Default
        match visualisation {
            Visualisation::Default => {
            if i < zone * zoom{            /* let zoom = 2; */
            let y = (i / (width))/zoom as usize;
            let x = ((i % (width))/zoom ) as usize;
                // let fit = self.members[y].fitness;
                // let alpha = 255 - (255.0 *  (zoom as f32 * fit as f32 /width as f32))as u32 ;
                let alpha = 255;

                // let genome = (zoom as f32 * genome_length as f32 * self.members[y].genome_sequence[x] as f32 / width as f32) as u8;
                let genome = (zoom as f32 * genome_length as f32 * self.members[y].genome_sequence[x] as f32 / width as f32) as u8;
                // pixel.copy_from_slice(&[genome, genome, genome, alpha as u8]);
                pixel.copy_from_slice(&[255, genome, 0, alpha as u8]);
              

            } else if i>= zone  * zoom&& i < 2 * zone * zoom{
                
                let x = ((i-width*pop_size*zoom) % (width))/zoom as usize;
                let y = ((i-(width*zoom*pop_size)) / (width))/zoom as usize;

                // let fit = self.members[y].fitness;
                let alpha = 255;

                let genome = (zoom as f32 * genome_length as f32 * self.members[y].female_chromosome[x] as f32 / width as f32) as u8;
                //
                /* pixel.copy_from_slice(&[0, genome, 255, alpha as u8]); */
                // pixel.copy_from_slice(&[255, genome, 0, alpha as u8]);
                // pixel.copy_from_slice(&[0, 255,0 , 255 as u8]);
                match config.organism_type {
                    OrganismType::Haploid => pixel.copy_from_slice(&[0, 0,0 , 255 as u8]),
                    OrganismType::Diploid => pixel.copy_from_slice(&[0, genome, 255, alpha as u8]),
                
                }
            } else if i >=2 *zone * zoom && i < 3*zone * zoom{ 
                let x = ((i-2*zone*zoom) % (width))/zoom as usize;
                let y = ((i-2*zone*zoom) / (width))/zoom as usize;

                // let fit = self.members[y].fitness;
                let alpha = 255;
                let epi =  10*self.members[y].epigenome[x];
                if epi >0 {
                    pixel.copy_from_slice(&[epi.abs() as u8, 0, 0, alpha as u8]);
                    pixel.copy_from_slice(&[255, 0, 0, alpha as u8]);
                } else if epi < 0 {
                    pixel.copy_from_slice(&[0, 0, epi.abs() as u8, alpha as u8]);
                    pixel.copy_from_slice(&[0, 0, 255, alpha as u8]);
                } else {
                    pixel.copy_from_slice(&[0, 255, 0, alpha as u8]);
                }
                // let genome = (zoom as f32 * genome_length as f32 * self.members[y].genome_sequence[x] as f32 / width as f32) as u8;

                // pixel.copy_from_slice(&[0, genome,0 , alpha as u8]);           
                // pixel.copy_from_slice(&[genome, genome, genome, 255 as u8]);
            
            } else {
                let k = i - (width * pop_size * zoom)*3;
                let y = (k / (width)) as usize;
                let ss = k - y * (zoom -1 ) * genome_length as usize;
                // if y == 1 {
                //     print!("{} {} ", k, ss);
                // }
                if ss <genome_length * genome_length{
                    let x = (k % (width))/zoom as usize;
                    let y = (k / (width))/zoom as usize;
                 
                    // let pix: u8 = (self.search_space[x+y*genome_length]) as u8;
                    let pig: u8 = (self.search_space[ss]/50.) as u8;
                    // let pib: u8 = (self.search_space[x+y*genome_length]/1024.) as u8;
                    let pis: u8 = (self.current_search_space[ss]) as u8;
                    // let pix: u8 = (self.search_space[ss] * 1.) as u8;
                    let pix: u8 = (self.search_space[ss]*1.) as u8;
                    if k % width <= genome_length {
                        pixel.copy_from_slice(&[pix, pix, pix, 255]);
                        pixel.copy_from_slice(&[pix, pig, pig, 255]);
                        // pixel.copy_from_slice(&[pig, pig, pix, 255]);
                    } else if k % width <= 2*genome_length {
                        pixel.copy_from_slice(&[pis, pis, pis, 255]);
                        // pixel.copy_from_slice(&[pig, pig, pix, 255]);

                    } else if k % width <= 3*genome_length {
                        pixel.copy_from_slice(&[pix*255, pix*255, pix*255, 255]);
                        // pixel.copy_from_slice(&[pig, pig, pix, 255]);
                     } else if k % width <= 4*genome_length {
                        pixel.copy_from_slice(&[pig, pig, pix, 255]);
                        // pixel.copy_from_slice(&[pig, pig, pix, 255]);                  
                        } 

                    else {
                        pixel.copy_from_slice(&[0, 0, 0, 255]);
                    }
                    if x*zoom == self.members[0].genome_sequence[y*zoom] {
                        // pixel.copy_from_slice(&[255, 255 ,255, 255]);
                    }
                    // pixel.copy_from_slice(&[pix, pig, pig, 255]);
                }
                // pixel.copy_from_slice(&[123, 123, 123, 155 as u8]);
            }
            
        }
        _ => {
            let y = (i / (width))/zoom as usize;
            let x = ((i % (width))/zoom ) as usize;
            // let y = i/(genome_length*zoom)/zoom;
            // let x = (i-y*genome_length*zoom)/zoom;
            let ss = y*genome_length+x;  
            if ss <self.current_search_space.len(){
            // let pis: u8 = (self.current_search_space[ss]) as u8;
            let pig: u8 = (self.search_space[ss]/50.) as u8;
            // let pib: u8 = (self.search_space[x+y*genome_length]/1024.) as u8;
            let pis: u8 = (self.current_search_space[ss]) as u8;
            // let pix: u8 = (self.search_space[ss] * 1.) as u8;
            let pix: u8 = (self.search_space[ss]*1.) as u8;

            match visualisation {
                Visualisation::Default => {
                    
                    // pixel.copy_from_slice(&[pix, pig, pig, 255]);
                }
                Visualisation::Fitness => {
                    pixel.copy_from_slice(&[pix, pig, pig, 255]);

                }
                Visualisation::SearchSpace => {
                    pixel.copy_from_slice(&[pix*255, pix*255, pix*255, 255]);


                }
                Visualisation::Optimal => {
                    pixel.copy_from_slice(&[pis, pis, pis, 255]);

                }
                Visualisation::Ultraviolet => {
                     pixel.copy_from_slice(&[pig, pig, pix, 255]);

                }
            }
                                                
                // pixel.copy_from_slice(&[pix*255, pix*255, pix*255, 255]);
            }
        }
        }
        }

    }



}

impl Dna {
    fn new(size: usize) -> Dna{
        let mut rng = rand::thread_rng();
        // let mut person = Vec::with_capacity(size);
        // let mut female_chromosome = Vec::with_capacity(size);
        // let mut male_chromosome = Vec::with_capacity(size);
        // for _ in 0..size {
        //     person.push(rng.gen_range(0..size));
        //     female_chromosome.push(rng.gen_range(0..size));
        //     male_chromosome.push(rng.gen_range(0..size));
        // }
        let mut person = Vec::from_iter(0..size);
        person.shuffle(&mut rng);
        // let mut male_chromosome: Vec<usize> = (0..size).collect();
        // let mut male_chromosome: Vec<usize> = vec![0; size];
        let mut m_chromosome: Vec<usize> = vec![0; size];
        let mut male_chromosome = Vec::from_iter(0..size);
        let mut female_chromosome: Vec<usize> = (0..size).collect();
        female_chromosome.shuffle(&mut rng);
        male_chromosome.shuffle(&mut rng);
        m_chromosome.shuffle(&mut rng);
        // male_chromosome.push(999);
        // female_chromosome.push(999);
        // let epigenome = vec![0;size];
        // let range = rand::thread_rng();
        // let epigenome: Vec<u32> = range.sample_iter(&Standard).map(|x: u32| x % 2).take(size).collect();
        let epigenome:Vec<i32> = vec![0;size];

        match rng.gen_range(0..2){
            0 => Dna { genome_sequence: person, female_chromosome, male_chromosome, m_chromosome, epigenome, sex: Sex::Male, fitness: 0, errors: 0, early:0, late:0},
            _ => Dna { genome_sequence: person, female_chromosome, male_chromosome, m_chromosome, epigenome, sex: Sex::Female, fitness: 0, errors: 0, early:0, late:0} 
        }
    }

}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DNA: \n{}, \nFitness: {}, \nEarly: {}, \nLate: {}, \nChromosme Length: {} \nChromosome:    {:?}\nMale Chromo:   {:?}\nFemale Chromo: {:?}\nEpigenome:     {:?}\n",
            self.sex, self.fitness, self.early, self.late, self.genome_sequence.len(), self.genome_sequence, self.male_chromosome, self.female_chromosome, self.epigenome
        )
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


impl Repair for  Vec<usize> {

    fn repair (&mut self){
        
        // Repair Dna
        // println!("Repairing");
        // let mut cpb = self.clone();
        // // cpb.sort();
        // println!("Before: {:?}", cpb);
        //
        let mut rng = rand::thread_rng();
        let genome_length = self.len();
        let mut ideal = Vec::from_iter(0..genome_length);
        let mut replace = Vec::<usize>::with_capacity(genome_length);

        for i in 0..genome_length{
            match ideal.contains(&self[i]) {
                true => {
                    let index = ideal.iter().position(|&r| r == self[i]).unwrap();
                    ideal.remove(index);
                }
                false => {
                    replace.push(i);
                }
                
            }
        }
        ideal.shuffle(&mut rng);
        for i in 0..replace.len(){
            let idx  = replace.pop().unwrap();
            self[idx] = ideal[i];
        }
        // let mut cpa = self.clone();
        // cpa.sort();
        // println!("After: {:?}", cpa);
        // panic!("Repairing Complete");
    }
}
impl Repair for  Dna {

    fn repair (&mut self){
        
        // Repair Dna
        // println!("Repairing");
        // let mut cpb = self.clone();
        // // cpb.sort();
        // println!("Before: {:?}", cpb);
        //
        let mut errors = 0;
        let mut rng = rand::thread_rng();
        let genome_length = self.genome_sequence.len();
        let mut ideal = Vec::from_iter(0..genome_length);
        let mut replace = Vec::<usize>::with_capacity(genome_length);

        for i in 0..genome_length{
            match ideal.contains(&self.genome_sequence[i]) {
                true => {
                    let index = ideal.iter().position(|&r| r == self.genome_sequence[i]).unwrap();
                    ideal.remove(index);
                }
                false => {
                    replace.push(i);
                    errors +=1;
                }
                
            }
        }
        self.errors = errors;
        ideal.shuffle(&mut rng);
        for i in 0..replace.len(){
            let idx  = replace.pop().unwrap();
            self.genome_sequence[idx] = ideal[i];
            self.male_chromosome[idx] = ideal[i];
            self.female_chromosome[idx] = ideal[i];
        }
        // let mut cpa = self.clone();
        // cpa.sort();
        // println!("After: {:?}", cpa);
        // panic!("Repairing Complete");
    }
}
impl fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Sex::Male => return write!(f,"Sex: Male  "),
            Sex::Female=> return write!(f,"Sex: Female")
         }

    }
}

impl Fitness for Population{
    fn calculate_fitness(&mut self, schedule: &Schedule, organism_type: &OrganismType) {
        let genome = self.members[0].genome_sequence.len();
        // let mut max_fitness = 0;
        // let mut min_fitness = 0;
        // let mut min_position = 0;
        // let mut min_chromosome = 0;
        match genome{
            yes if yes == schedule.tasks.len() => {

                
            for i in 0..self.members.len(){
                // println!("Cecking fitness of member {i}");
                self.members[i].calculate_fitness(&schedule, &organism_type);

                // Additional code for epigenetic algorithm
                // if i == 0{
                //     max_fitness = self.members[i].female_chromosome[genome];
                //     min_fitness = self.members[i].female_chromosome[genome];
                // }
                // if self.members[i].female_chromosome[genome] > max_fitness{
                //     max_fitness =self.members[i].female_chromosome[genome]; 
                // }
                //  if self.members[i].male_chromosome[genome] > max_fitness{
                //     max_fitness =self.members[i].male_chromosome[genome]; 
                // }               // println!("Cecking fitness of member {i} = {} ", self.member[i].fitness);
                //  if self.members[i].female_chromosome[genome] < min_fitness{
                //     min_fitness =self.members[i].female_chromosome[genome]; 
                //     // min_position = i;
                //     // min_chromosome = 0;
                // }
                //  if self.members[i].male_chromosome[genome] < min_fitness{
                //     min_fitness =self.members[i].male_chromosome[genome]; 
                //     // min_position = i;
                //     // min_chromosome = 1;
                // }              
            }
            // println!("Max Fitness: {}", max_fitness);
            // println!("Min Fitness: {}", min_fitness);    
            // println!("Min Position: {}", min_position);
            // println!("Min Chromosome: {}", min_chromosome);
            // println!("Min Fitness: {}", self.members[min_position]);

                // println!("Gene Checking: Passed")
            },
            _           => panic!("MISMACTHED GENOME SEQUENCES")
        }

    }
}






// Concurrency method - wasn't faster
//
// impl Fitness for Population {
//     fn calculate_fitness(&mut self, schedule: &Schedule) {
//         let genome = self.members[0].genome_sequence.len();
//
//         match genome {
//             yes if yes == schedule.tasks.len() => {
//                 self.members.par_iter_mut().for_each(|member| {
//                     member.calculate_fitness(&schedule);
//                 });
//             }
//             _ => panic!("MISMATCHED GENOME SEQUENCES"),
//         }
//     }
// }




impl Breed for Population{
    fn breed(&mut self,top: usize, _schedule: Schedule, organism_type: &OrganismType, config: &GeneticAlgorithmConfig) {

        self.evolution.push(self.members[0].fitness);
        let mut rng = rand::thread_rng();
        let genome_length = self.members[0].genome_sequence.len();
        let mut next_generation = Population{members:Vec::with_capacity(self.members.len()), evolution: self.evolution.clone(),
                                    search_space: vec![0.0;genome_length*genome_length],current_search_space: vec![0.0;genome_length*genome_length], dominance: vec![0;genome_length]};

        let keepers = self.members.len()*top/100;      
        // println!("Breeding: ");
        // println!("Top Selected for Breeding: {}", keepers);
        for i in 0..keepers {
            let parent1 = &self.members[i];
            for _ in 0..100/top*2{
                match organism_type {
                    OrganismType::Haploid => {
                        let mut child = Dna::new(parent1.genome_sequence.len());
                        child.genome_sequence = parent1.genome_sequence.clone();

                        // let mutationed_gene = rng.gen_range(0..genome_length);

                        // Crossover Method 1
                        // let crossover_point = rng.gen_range(0..genome_length);
                        // let crossover_point2 = rng.gen_range(0..genome_length);
                        // child.genome_sequence.swap(crossover_point, crossover_point2);
                        match config.crossover_method{
                            CrossoverMethod::SinglePoint => {
                                let crossover_point = rng.gen_range(0..genome_length);
                                let crossover_point2 = rng.gen_range(0..genome_length);
                                child.genome_sequence.swap(crossover_point, crossover_point2);
                            },
                            CrossoverMethod::Epigenetic => {
                                // Crossover Method 2
                                let mut crossover_point = rng.gen_range(0..genome_length);
                                let mut crossover_point2 = 0;

                                for i in crossover_point..parent1.genome_sequence.len(){
                                        if parent1.epigenome[i] < 0 {
                                            // println!("Epigenome: {} {:?}", i, parent1.epigenome[i]);
                                            crossover_point = i;
                                            break;
                                        }
                                }
                                for j in crossover_point+1..parent1.genome_sequence.len(){
                                        if parent1.epigenome[j] > 0 && j -crossover_point <=parent1.epigenome[j] as usize{
                                            if j > crossover_point2{
                                                crossover_point2 = j;
                                            }
                                        
                                            // break;
                                        }
                                        if parent1.epigenome[j] > 0 && self.members[0].fitness <200{
                                            // println!("Epigenome: {} {:?}", i, parent1.epigenome[i]);
                                            crossover_point2 = j;
                                            break;
                                        }
                                }
                                if config.diagnostics == true {
                                    println!("Crossover Point [{}]: {} {} -- {} {} ::: {} {}"
                                             ,next_generation.members.len(), crossover_point, crossover_point2
                                             ,parent1.epigenome[crossover_point], parent1.epigenome[crossover_point2],
                                             parent1.genome_sequence[crossover_point], parent1.genome_sequence[crossover_point2]);
                                }
                                child.genome_sequence.swap(crossover_point, crossover_point2);
                                if rng.gen_range(0..100) < (config.mutation_rate * 100.) as u64 {
                                    let crossover_point = rng.gen_range(0..genome_length);
                                    while parent1.epigenome[crossover_point] == 0 {
                                        let crossover_point = rng.gen_range(0..genome_length);
                                        if parent1.epigenome[crossover_point] != 0 {
                                            break;
                                        }
                                    }
                                    let crossover_point2 = rng.gen_range(0..genome_length);
                                    while parent1.epigenome[crossover_point2] == 0 {
                                        let crossover_point2 = rng.gen_range(0..genome_length);
                                        if parent1.epigenome[crossover_point2] != 0 {
                                            break;
                                        }
                                    }
                                    child.genome_sequence.swap(crossover_point, crossover_point2);
                                }
                                },
                            _ => {
                            
                            }


                        } 
                        if next_generation.members.len() >= self.members.len() {
                            break;
                        } 
                        next_generation.members.push(child);
                        
                    },
                    OrganismType::Diploid => {
              
                        let mut child = Dna::new(parent1.genome_sequence.len());
                        // let selected_organism = rng.gen_range(top..self.members.len());
                        // let selected_organism = rng.gen_range(0..self.members.len());
                        let selected_organism = rng.gen_range(0..10);
                        let parent2 = &self.members[selected_organism];
                        // 
                        // child.genome_sequence = parent1.genome_sequence.clone();
                        child.male_chromosome = parent1.male_chromosome.clone();
                        child.female_chromosome = parent2.female_chromosome.clone();
                        // child.epigenome = parent1.epigenome.clone();

                        let crossover_point = rng.gen_range(0..genome_length);
                        let crossover_point2 = rng.gen_range(0..genome_length);
                        //
                        child.female_chromosome.swap(crossover_point, crossover_point2);
                        
                        let crossover_point = rng.gen_range(0..genome_length);
                        let crossover_point2 = rng.gen_range(0..genome_length);
                        child.male_chromosome.swap(crossover_point, crossover_point2);






                        // let chance = rng.gen_range(0..100);
                        // if chance == 1{
                        // let mutation_start = rng.gen_range(0..genome_length-3);
                        // let mutation_end = rng.gen_range(mutation_start+1..genome_length-2);
                        //
                        // let swap = child.male_chromosome[i];
                        // for i in  mutation_start..mutation_end-1{
                        //     // println!("Swapping {} & {} :: St {} Ed {}",i,i+1,mutation_start,mutation_end);
                        //     child.male_chromosome[i] = child.male_chromosome[i+1];
                        // }
                        // child.male_chromosome[mutation_end] = swap;
                        // }
                        
                        // let mutationed_gene = rng.gen_range(0..genome_length-1);
                        // child.male_chromosome.swap(i,i+1);





                        // child.male_chromosome.repair();
                        // child.female_chromosome.repair();
                        // let crossover_point = rng.gen_range(0..genome_length);
                        // 'outer: for j in 0..child.genome_sequence.len(){
                        // let j = rng.gen_range(0..genome_length);
                        //     if child.epigenome[j] < 0 {
                        //
                        //         for k in j+1..child.genome_sequence.len(){
                        //             if child.epigenome[k] > 0{
                        //                 // println!("Swapping {} & {}",j,k);
                        //                 // println!("Before: {:?}", child.female_chromosome);
                        //                 // if i==0{
                        //                 //     println!("Before: {:?}", child.female_chromosome);
                        //                 // }
                        //                 child.female_chromosome.swap(j,k);
                        //                 println!("{}: Swapping {} & {}",i,j,k);
                        //                 // if i ==0 {
                        //                 //     println!("After: {:?}", child.female_chromosome);
                        //                 // }
                        //                 // child.epigenome[i] = 0;
                        //                 // child.epigenome[j] = 0;
                        //                 // break 'outer;
                        //                 break;
                        //             }
                        //         }
                        //     }
                        // }


                        // child.epigenome = parent1.epigenome.clone();
                        // child.m_chromosome = parent2.m_chromosome.clone();
                        //
                        //
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // child.genome_sequence[mutationed_gene] = rng.gen_range(0..genome_length);
                        // child.female_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        //
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // child.male_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        // 
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // child.m_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        // 
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // child.epigenome[mutationed_gene] = rng.gen_range(0..2);
                        // child.female_chromosome.repair();
                        // child.male_chromosome.repair();
                        // child.m_chromosome.repair();

                        // for i in 0..child.genome_sequence.len(){
                        //     if child.epigenome[i] == 0{
                        //     // if i < genome_length/20000{
                        //         child.genome_sequence[i] = child.female_chromosome[i];
                        //     } else {
                        //         child.genome_sequence[i] = child.male_chromosome[i];
                        //     }
                        //     
                        // }
                        // child.genome_sequence = child.m_chromosome.clone();
                        // child.genome_sequence.repair();
                        
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // child.epigenome[mutationed_gene] = rng.gen_range(0..2);
                        // child.epigenome = vec![0;genome_length];
                        
                        // let mut child = Dna::new(parent1.genome_sequence.len());
                        // child.male_chromosome = parent1.male_chromosome.clone();
                        // child.female_chromosome = parent1.female_chromosome.clone();
                        // child.epigenome = parent1.epigenome.clone();
        
                        // crossover
                        // let crossover_point = rng.gen_range(1..genome_length);
                        // for k in crossover_point..10{
                        //     child.male_chromosome[k] = parent1.female_chromosome[k];
                        // }
                        //  let crossover_point = rng.gen_range(1..genome_length);
                        // for k in crossover_point..10{
                        //     child.female_chromosome[k] = parent2.female_chromosome[k];
                        // }
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // let mutationed_gene2 = rng.gen_range(0..genome_length);
                        // child.female_chromosome.swap(mutationed_gene2, mutationed_gene);
                        // child.female_chromosome.repair();
                        // child.male_chromosome.repair();

                        // if child.epigenome[mutationed_gene] == 0 {
                        //     // child.male_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        //     // child.male_chromosome.repair();
                        // } else {
                        //     // child.female_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        //     // child.female_chromosome.repair();
                        // }
                        // child.male_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        // child.male_chromosome.repair();

                        // let crossover_point = rng.gen_range(1..genome_length);
                        // for k in crossover_point..10{
                        //     child.female_chromosome[k] = parent2.female_chromosome[k];
                        // }                   
                       
                        // let mutationed_gene = rng.gen_range(0..genome_length);
                        // child.female_chromosome[mutationed_gene] = rng.gen_range(0..genome_length);
                        // child.female_chromosome.repair();

                        // for i in 0..child.genome_sequence.len(){
                        //     if child.epigenome[i] == 0{
                        //     // if i < genome_length/2{
                        //         child.genome_sequence[i] = child.male_chromosome[i];
                        //     } else {
                        //         child.genome_sequence[i] = child.female_chromosome[i];
                        //     }
                        //     
                        // }
                        // child.genome_sequence.repair();
                        if next_generation.members.len() >= self.members.len() {
                            break;
                        } 
                        next_generation.members.push(child);

                   }
               }
                   
           }
        }
        // println!("Post Breeding Population: {}", next_generation.members.len()); 
        // copy search space into next generation
        for i in 0..genome_length*genome_length {
            next_generation.search_space[i] = self.search_space[i];
        }
        // 
        // Add the search of the previous generation to the search space
        for i in 0..self.members.len(){
            // for j in 0..self.members[i].genome_sequence.len(){
            for j in 0..self.members[i].female_chromosome.len(){
                let search_loc = j * genome_length + self.members[i].genome_sequence[j];
                // let search_loc = j * genome_length + self.members[i].female_chromosome[j];
                let prev = self.search_space[search_loc] ;
                next_generation.search_space[search_loc] =  prev + 2.55;
                next_generation.current_search_space[search_loc] = 255.;
            }
        }

        // Keep the best organism from the previous generation in the new generation 
        let best = keepers;
        // println!("Best: {} of {}",best,next_generation.members.len());
        for _ in 0..best{
            next_generation.members.pop();
            // println!("POP :: {:?}",next_generation.members.pop());
        }
        // println!("Post Pop: {}", next_generation.members.len());
        // println!("\nBest: {}", best);
        for i in 0..best{
            let mut last_best = Dna::new(genome_length);
            last_best.genome_sequence   = self.members[i].genome_sequence.clone();
            last_best.epigenome         = self.members[i].epigenome.clone();
            last_best.male_chromosome   = self.members[i].male_chromosome.clone();
            last_best.female_chromosome = self.members[i].female_chromosome.clone();
            // last_best.fitness           = self.members[i].fitness;
            // last_best.early             = self.members[i].early;
            // last_best.late              = self.members[i].late;
            // last_best.m_chromosome      = self.members[i].m_chromosome.clone();
            next_generation.members.push(last_best);
        }
        // print!("{},",next_generation.members[self.members.len()-best].fitness);
        // print!("\n");
        assert_eq!(next_generation.members.len(),self.members.len());
        // println!("Next Generation: {:?}", next_generation.members[0].female_chromosome);
        *self = next_generation;
    }

}

trait Repair {
    fn repair(&mut self);
}

trait Breed {
    fn breed(&mut self,top:usize,schedule: Schedule, organism_type: &OrganismType, config: &GeneticAlgorithmConfig);
}

pub trait Fitness {
    fn calculate_fitness(&mut self, schedule: &Schedule,organism_type: &OrganismType);
}


// User implemented Trait for fitness of DNA 
impl Fitness for Dna {
    fn calculate_fitness(&mut self, schedule: &Schedule,organism_type: &OrganismType) {
        let mut score = 0;
        let mut early = 0;
        let mut late = 0;
        let mut start_date = 0;
        let mut workshop_id;
        let mut start_dates = vec![0;8];
        match organism_type {
            OrganismType::Haploid =>{
                for i in 0..self.genome_sequence.len(){

                    if let Some(task) = schedule.tasks.get(self.genome_sequence[i]) {
                        match task.workshop.as_str() {
                            "ELEC" => {workshop_id = 0;},
                            "FAB" => {workshop_id = 1;},
                            "HYD" => {workshop_id = 2;},
                            "MECH" => {workshop_id = 3;},
                            "M/C" => {workshop_id = 4;},
                            "NDT" => {workshop_id = 5;},
                            "P&M" => {workshop_id = 6;},
                            "PAINT" => {workshop_id = 7;},
                            _ => {workshop_id = 0;}
                        }
                        start_dates[workshop_id] = start_dates[workshop_id] + task.ctr;
                        start_date = start_dates[workshop_id];
                        // start_date = start_date + task.ctr;        
                        let task_deadline = task.deadline_days.unwrap();
                        if task_deadline < start_date {
                            late = late + (start_date - task_deadline);
                            score = score + (start_date - task_deadline);
                            self.epigenome[i] = (start_date - task_deadline) as i32;
                        }
                        if task_deadline > start_date {
                            score = score + (task_deadline - start_date);
                            early = early + (task_deadline - start_date);
                            self.epigenome[i] = (start_date - task_deadline) as i32;
                        }
                        } else {
                            println!("Missing Task No.{}",&self.genome_sequence[i]);
                            panic!("Task not found");
                        }
                }
                self.fitness = score;
                self.early = early;
                self.late = late;

                
            },
            OrganismType::Diploid =>{
                // let mut start_date_epi = 0;
                // for i in 0..self.genome_sequence.len(){
                //     if let Some(task) = schedule.tasks.get(self.genome_sequence[i]) {
                //
                //         start_date_epi = start_date_epi + task.ctr;        
                //         let task_deadline = task.deadline_days.unwrap();
                //         if task_deadline < start_date_epi {
                //             // late = late + (start_date_epi - task_deadline);
                //             // score = score + (start_date_epi - task_deadline);
                //             self.epigenome[i] = (start_date_epi - task_deadline) as i32;
                //         }
                //         if task_deadline > start_date_epi {
                //             // score = score + (task_deadline - start_date_epi);
                //             // early = early + (task_deadline - start_date_epi);
                //             self.epigenome[i] = (start_date_epi - task_deadline) as i32;
                //         }
                //         } else {
                //             println!("Missing Task No.{}",&self.male_chromosome[i]);
                //             panic!("Task not found");
                //         }
                // }
                // // self.male_chromosome[self.genome_sequence.len()] = score as usize;
                // let male_score = score;
                // score = 0;
                // start_date = 0;
                // for i in 0..self.genome_sequence.len(){
                //
                //     if let Some(task) = schedule.tasks.get(self.female_chromosome[i]) {
                //
                //         start_date = start_date + task.ctr;        
                //         let task_deadline = task.deadline_days.unwrap();
                //         if task_deadline < start_date {
                //             late = late + (start_date - task_deadline);
                //             score = score + (start_date - task_deadline);
                //         }
                //         if task_deadline > start_date {
                //             score = score + (task_deadline - start_date);
                //             early = early + (task_deadline - start_date);
                //         }
                //         } else {
                //             println!("Missing Task No.{}",&self.female_chromosome[i]);
                //             panic!("Task not found");
                //         }
                // }
                // // self.female_chromosome[self.genome_sequence.len()] = score as usize;
                // let female_score = score;
                // // build expressed genome sequence 
                // for i in 0..self.genome_sequence.len(){
                //     if self.epigenome[i] == 0{
                //         self.genome_sequence[i] = self.male_chromosome[i];
                //     } else {
                //         self.genome_sequence[i] = self.female_chromosome[i];
                //     }
                //     
                // }
                // self.genome_sequence.repair();
                // score = 0;
                // start_date = 0;         
                //
                
                // Potential for Diplod fitness function
                // ISSUE: Can generate invalid genome sequences with double genes and missing genes
                let mut task_c1: &Task;
                let mut task_c2: &Task;

                for i in 0..self.genome_sequence.len(){
                    if let Some(task) = schedule.tasks.get(self.female_chromosome[i]){
                         task_c1 = task;
                    } else {
                        panic!("Task not found");
                    };
                    if let Some(task) = schedule.tasks.get(self.male_chromosome[i]){
                        task_c2 = task;
                    } else {
                        panic!("Task not found");
                    };
                    let start_date_c1 = start_date + task_c1.ctr;
                    let task_deadline = task_c1.deadline_days.unwrap();
                    let mut score_c1 = score;
                    let mut early_c1 = early;
                    let mut late_c1 = late;
                    let mut score_c2 = score;
                    let mut early_c2 = early;
                    let mut late_c2 = late;

                    if task_deadline < start_date_c1 {
                        late_c1 = late + (start_date_c1 - task_deadline);
                        score_c1 = score + (start_date_c1 - task_deadline);
                        // score = score + (start_date_c1 - task_deadline);
                    }
                    if task_deadline > start_date {
                        score_c1 = score + (task_deadline - start_date_c1);
                        early_c1 = early + (task_deadline - start_date_c1);
                        // score = score + (task_deadline - start_date_c1);
                    }
                    
                    let start_date_c2 = start_date + task_c2.ctr;
                    let task_deadline = task_c2.deadline_days.unwrap();
                    if task_deadline < start_date_c2 {
                        late_c2 = late + (start_date_c2 - task_deadline);
                        score_c2 = score + (start_date_c2 - task_deadline);
                        // score = score + (start_date_c2 - task_deadline);
                    }
                    if task_deadline > start_date {
                        score_c2 = score + (task_deadline - start_date_c2);
                        early_c2 = early + (task_deadline - start_date_c2);
                        // score = score + (task_deadline - start_date_c2);
                    }
                    if score_c1 < score_c2 {
                    // if i < self.genome_sequence.len()/2 {
                    // if true {
                        score =  score_c1;
                        start_date = start_date_c1;
                        early = early_c1;
                        late = late_c1;
                        self.genome_sequence[i] = self.female_chromosome[i];
                        // println!("c1<c2: {} < {}",score_c1,score_c2);
                    } else {
                        score =  score_c2;
                        start_date = start_date_c2;
                        early = early_c2;
                        late = late_c2;
                        self.genome_sequence[i] = self.male_chromosome[i];
                        // println!("c2<c1: {} < {}",score_c2,score_c1);
                    }
                }

                // self.fitness = score/2;
                // self.early = early;
                // self.late = late;
                //
                // let mut rng = rand::thread_rng();
                // let chance = rng.gen_range(0..3);
                // if chance == 0 {
                //     self.repair();
                // }
                    self.repair();
 // 
            // }
                // score = 0;
                // early = 0;
                // late = 0;      
                // for i in 0..self.genome_sequence.len(){
                //
                //     if let Some(task) = schedule.tasks.get(self.male_chromosome[i]) {
                //
                //         start_date = start_date + task.ctr;        
                //         let task_deadline = task.deadline_days.unwrap();
                //         if task_deadline < start_date {
                //             late = late + (start_date - task_deadline);
                //             score = score + (start_date - task_deadline);
                //         }
                //         if task_deadline > start_date {
                //             score = score + (task_deadline - start_date);
                //             early = early + (task_deadline - start_date);
                //         }
                //         } else {
                //             println!("Missing Task No.{}",&self.genome_sequence[i]);
                //             panic!("Task not found");
                //         }
                // }
                //
                //
                let mut start_date_epi = 0;
                for i in 0..self.genome_sequence.len(){
                    if let Some(task) = schedule.tasks.get(self.genome_sequence[i]) {

                        start_date_epi = start_date_epi + task.ctr;        
                        let task_deadline = task.deadline_days.unwrap();
                        if task_deadline < start_date_epi {
                            // late = late + (start_date_epi - task_deadline);
                            // score = score + (start_date_epi - task_deadline);
                            self.epigenome[i] = (start_date_epi - task_deadline) as i32;
                        }
                        if task_deadline > start_date_epi {
                            // score = score + (task_deadline - start_date_epi);
                            // early = early + (task_deadline - start_date_epi);
                            self.epigenome[i] = (start_date_epi - task_deadline) as i32;
                        }
                        } else {
                            println!("Missing Task No.{}",&self.male_chromosome[i]);
                            panic!("Task not found");
                        }
                }

                
                self.fitness = score;
                self.early = early;
                self.late = late;
                }           

        
        }

    }
}



fn info (config: &GeneticAlgorithmConfig, schedule: &Schedule) {
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

    println!("Threads Available: {}", num_cpus::get());
    // println!("???"); 
    println!(""); 
    println!("--------------------------------"); 
    println!("Genetic Algoritm Version: {}", VERSION); 
    println!("--------------------------------"); 
    println!("");
    println!("----------- SETTINGS -----------");
    println!("Population Size:  {}", config.population_size);
    println!("Genome Length:    {}", schedule.tasks.len());
    println!("Mutation Rate:    {}%", config.mutation_rate);
    println!("Organism Type:    {:?}", config.organism_type);
    println!("Crossover Method: {:?}", config.crossover_method);
    println!("Mutation Method:  {:?}", config.mutation_method);
    println!("Generation Limit: {}", config.max_generations);
    println!("--------------------------------"); 
    println!("");
}

pub enum Visualisation {
    Default,
    Fitness,
    Optimal,
    Ultraviolet,
    SearchSpace,
}

pub struct GeneticAlgorithmConfig {
    pub population_size: usize,
    pub mutation_rate: f64,
    pub organism_type: OrganismType,
    pub crossover_method: CrossoverMethod,
    pub mutation_method: MutationMethod,
    pub max_generations: u64,
    pub visualise: bool,
    pub diagnostics: bool,
    }
#[derive(Debug)]
pub enum CrossoverMethod {
    None,
    Uniform,
    SinglePoint,
    Epigenetic,
}

#[derive(Debug)]
pub enum MutationMethod {
    None,
    Uniform,
    SinglePoint,
}

#[derive(Debug)]
pub enum OrganismType {
    Diploid,
    Haploid,
}

fn coverage(search_space: &Vec<f32>) -> f32{
    let mut cover = 0;

    for i in 0..search_space.len(){
        if search_space[i] != 0. {
            cover +=1;
        }
    }
    let cover = cover as f32;
    let search_space = search_space.len() as f32;
    let coverage = 100.*cover/search_space;
    coverage
}

fn visual_simulation(schedule: &Schedule, mut config: GeneticAlgorithmConfig) -> Result<Population, Error>{


    info(&config, &schedule);
    // let base_schedule = schedule.clone();
    // schedule.tasks.pop();
    // 
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    // schedule.tasks.pop();
    let mut schedule = schedule.clone();
    let genome_length = schedule.tasks.len();
    let population_size = config.population_size;
    let start = Instant::now();
    let mut world = Population::new(population_size, genome_length);
    let duration = start.elapsed();
    let mut first = true;
    let mut step = false;
    let mut zoom = 3;
    let mut play = false; //true;

    println!("Time elapsed to generate population of {} x {} is: {:?}", population_size, genome_length, duration);

    world.calculate_fitness(&schedule,&config.organism_type);
    // // world.members.sort();
    world.sort_members_by_fitness(); 
    // let mut i = 0;


    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        // let size = LogicalSize::new((genome_length * zoom) as f64, (population_size * zoom + genome_length) as f64);
        let size = LogicalSize::new((genome_length * zoom ) as f64, (population_size * zoom * 3+genome_length) as f64);
        WindowBuilder::new()
            .with_title("Genetic Algorithm Population")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_position(winit::dpi::PhysicalPosition::new(50,200))
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new((zoom * genome_length) as u32, (population_size * zoom * 3 + genome_length) as u32, surface_texture)?
    };


    // let  evolution: Vec<usize> = Vec::with_capacity(1000);
    let mut i = 0;
    let mut visual = Visualisation::Default;
    // loop {
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut(),zoom,&config,&visual);
            if let Err(_) = pixels.render() {
                // log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }


        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                println!("\nSimulation Ended by User");
                return;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                println!("\nSimulation Paused by User");
                play = !play;
                step = false;
                *control_flow = ControlFlow::Wait;
                // return;
            }
            if input.key_pressed(VirtualKeyCode::Return) {
                println!("\nSimulation Resumed by User");
                play = true;
                step = false;
                *control_flow = ControlFlow::Poll;
                return;
            }
            if input.key_pressed(VirtualKeyCode::M) {
                play = false;
                println!("\nMenu Options");
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::S) {
                play = true;
                step = true;
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                println!("\nFittest member:\n {}", world.members[0]);
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::F) {
                for i in 0..world.members.len(){
                    println!("Fitness: {}", world.members[i].fitness);
                }
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::R) {
                println!("\nReversing Schedule");
                schedule.tasks.reverse();
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::C) {
                println!("\nClear Search Space:");
                world.search_space = vec![0.0;genome_length*genome_length];
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::D) {
                println!("\nDisplaying Search Space:");
                config.organism_type = OrganismType::Diploid;
                // *control_flow = ControlFlow::E:xit;
                world.search_space = vec![0.0;genome_length*genome_length];
                return;
            }
            if input.key_pressed(VirtualKeyCode::H) {
                println!("\nDisplaying Search Space:");
                config.organism_type = OrganismType::Haploid;
                world.search_space = vec![0.0;genome_length*genome_length];
                // *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Key0) {
                visual = Visualisation::Default;
                // *control_flow = ControlFlow::Exit;
                window.request_redraw();
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key1) {
                visual = Visualisation::Fitness;
                // *control_flow = ControlFlow::Exit;
                window.request_redraw();
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key2) {
                visual = Visualisation::Optimal;
                window.request_redraw();
                // *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key3) {
                visual = Visualisation::Ultraviolet;
                // *control_flow = ControlFlow::Exit;
                window.request_redraw();
                return;
            }
            if input.key_pressed(VirtualKeyCode::Key4) {
                visual = Visualisation::SearchSpace;
                // *control_flow = ControlFlow::Exit;
                window.request_redraw();
                return;
            }
            if input.key_pressed(VirtualKeyCode::Comma) {
                // zoom += 1;
                // println!("Zoom: {}",zoom);
                // window.request_redraw();
                config.mutation_rate = 0.9;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Minus) {
                // config.mutation_rate = cmp::min(0,100 * config.mutation_rate as u64 - 1 ) as f64 / 100. ;
                config.mutation_rate = 0.;
            //     zoom = cmp::max(1,zoom-1);
            //     println!("Zoom: {}",zoom);
            //     window.res
            //     pixels = {
            //         let window_size = window.inner_size();
            //         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            //         Pixels::new((zoom * genome_length) as u32, (population_size * zoom * 3 + genome_length) as u32, surface_texture).unwrap()
            // };
            //     window.request_redraw();
                return;
            }


            // // Resize the window
            // if let Some(size) = input.window_resized() {
            //     if let Err(err) = pixels.resize_surface(size.width, size.height) {
            //         log_error("pixels.resize_surface", err);
            //         *control_flow = ControlFlow::Exit;
            //         return;
            //     }
            // }


     // loop  
     if play {
        i=i+1;
        // world.calculate_fitness(&schedule,&config.organism_type);
        // world.members.sort();
        // world.sort_members_by_fitness();
        // let schedule_copy = schedule.clone();
        world.breed(10,schedule.clone(),&config.organism_type,&config);
        window.request_redraw();
        if step == true {
            play = false;
        }
        world.calculate_fitness(&schedule,&config.organism_type);
        world.sort_members_by_fitness(); 
        let pcover = coverage(&world.search_space);
        // println!("{}: Min Fitness {}=={:?}, Early: {} :: Late {}, Size: {:?} x {}  Coverage: {:.2}%    \r",i,world.members[0].fitness,world.find_minimum_fitness(), world.members[0].early, world.members[0].late, world.members[0].genome_sequence.len(), world.members.len(), pcover); for j in 0..world.members.len(){
        //     print!("\nMember {},{}",j,world.members[j]);
        // }
        // println!("{}",world.members[0]);
        // panic!("Stop");
        // let dominance:usize = world.dominance.iter().sum();
        if first == true {
            print!("{}: Min Fitness {}, Early: {} :: Late {}, Mutation Rate: {}, Size: {:?} x {}  Coverage: {:.2}%    \r",i,world.members[0].fitness, 
                   world.members[0].early, world.members[0].late, config.mutation_rate, world.members[0].genome_sequence.len(), world.members.len(), pcover);
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
            world.evolution.push(world.members[0].fitness);
            // println!("Evolution: {:?}",world.evolution);
            // world.evolution.export_to_csv("optimal.csv")
            //         .expect("Failed to export evolution to csv");           first = false
            first = false;
            play = false;
            window.request_redraw();
            // return world;
        }
        if i > config.max_generations {
            first = false;
            play = false;           
            window.request_redraw();
            println!();
            println!("Solution not found in {} generations",i);
            return ;
        }}
        }
    });
    // return Ok(world);
}
fn simulation(schedule: &Schedule, config: GeneticAlgorithmConfig) -> Result<Population, Error>{


    info(&config, &schedule);
    let genome_length = schedule.tasks.len();
    let population_size = config.population_size;
    let start = Instant::now();
    let mut world = Population::new(population_size, genome_length);
    let duration = start.elapsed();
    let first = true;

    println!("Time elapsed to generate population of {} x {} is: {:?}", population_size, genome_length, duration);

    world.calculate_fitness(&schedule,&config.organism_type);
    // // world.members.sort();
    world.sort_members_by_fitness(); 

    // let  evolution: Vec<usize> = Vec::with_capacity(1000);
    let mut i = 0;
    
    loop{

        i=i+1;
        // world.calculate_fitness(&schedule,&config.organism_type);
        // world.members.sort();
        // world.sort_members_by_fitness();
        // let schedule_copy = schedule.clone();
        world.breed(20,schedule.clone(),&config.organism_type,&config);
        world.calculate_fitness(&schedule,&config.organism_type);
        world.sort_members_by_fitness(); 
        let pcover = coverage(&world.search_space);
        if first == true {
            print!("{}: Min Fitness {}, Early: {} :: Late {}, Errors: {}, Size: {:?} x {}  Coverage: {:.2}%    \r",i,world.members[0].
                   fitness, world.members[0].early, world.members[0].late, world.members[0].errors, world.members[0].genome_sequence.len(), world.members.len(), pcover);
        }

        if world.members[0].fitness ==0 && first == true{
        // Optimal solution found
            println!();
            println!("Solution found in generation {}",i);
            println!("{}: Min Fitness {}, Early: {} :: Late {}, Size: {:?} x {}  Coverage: {:.2}%    \r",i,world.members[0].fitness, world.members[0].early, world.members[0].late, world.members[0].genome_sequence.len(), world.members.len(), pcover);
            let duration = start.elapsed();
            println!("Simulation completed in {:?}", duration);            
            world.evolution.push(world.members[0].fitness);
            // world.evolution.export_to_csv("optimal.csv")
            //         .expect("Failed to export evolution to csv");           first = false
            break;
        }
        if i > config.max_generations {
            // Time out
            println!();
            println!("Solution not found in {} generations",i);
            break;
        }
     }
     return Ok(world);
}

pub fn genetics(schedule: &Schedule, config: GeneticAlgorithmConfig) -> Result<Population, Error>{
    
    let solution;
    match config.visualise {
        true => {
            solution = visual_simulation(schedule, config)
        },
        false => {
            solution = simulation(schedule, config)
        }
    }
    Ok(solution?)
}
