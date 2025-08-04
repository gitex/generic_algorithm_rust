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

    pub fn iter(&self) -> impl Iterator<Item = &Genome> {
        self.population.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Genome> {
        self.population.iter_mut()
    }

    // pub fn mutate<R: Rng + ?Sized>(&mut self, rng: &mut R) {
    //     for genome in self.iter_mut() {
    //         genome.mutate(rng);
    //     }
    // }
}

fn main() {
    let mut rng = rand::rng();

    let options = Options { min: 0, max: 1 };

    let genomes = (0..50).map(|_| {
        Genome::new(vec![Chromosome::new(vec![
            Gene::new("gene1", options),
            Gene::new("gene2", options),
            Gene::new("gene3", options),
            Gene::new("gene4", options),
            Gene::new("gene5", options),
            Gene::new("gene6", options),
            Gene::new("gene7", options),
            Gene::new("gene8", options),
            Gene::new("gene9", options),
        ])])
        .build(&mut rng)
    });

    let population = Population::new(genomes.collect());

    for genome in population.best_genomes(5) {
        println!("Best Genome: {:?}", genome.fitness());
    }

    // println!(":?", genome.clone());
}
