#[allow(unused_imports)]
use rand::Rng;

mod structs;

use structs::{Build, Chromosome, Fitness, Gene, Genome, Options};

impl Fitness for Genome {
    fn fitness(&self) -> f64 {
        let mut weight = 0.0; // Placeholder for weight calculation

        for chromosome in self.iter() {
            for gene in chromosome.iter() {
                weight += *gene.value().unwrap() as f64;
            }
        }

        weight
    }
}

pub struct Population {
    population: Vec<Genome>,
}

impl Population {
    pub fn new(population: Vec<Genome>) -> Self {
        Population { population }
    }

    pub fn best_genomes(&self, count: usize) -> Vec<Genome> {
        let mut genomes = self.population.clone();
        genomes.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        genomes.into_iter().take(count).collect()
    }
}

fn main() {
    let mut rng = rand::rng();

    let genomes = (0..50).map(|_| {
        Genome::new(vec![Chromosome::new(vec![
            Gene::new("gene1", Options { min: 0, max: 1 }),
            Gene::new("gene2", Options { min: 0, max: 1 }),
            Gene::new("gene3", Options { min: 0, max: 1 }),
            Gene::new("gene4", Options { min: 0, max: 1 }),
            Gene::new("gene5", Options { min: 0, max: 1 }),
            Gene::new("gene6", Options { min: 0, max: 1 }),
            Gene::new("gene7", Options { min: 0, max: 1 }),
        ])])
        .build(&mut rng)
    });

    let population = Population::new(genomes.collect());

    for genome in population.best_genomes(5) {
        println!("Best Genome: {:?}", genome.fitness());
    }

    // println!(":?", genome.clone());
}
