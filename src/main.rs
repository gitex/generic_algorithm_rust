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

    pub fn run(&mut self, iterations: usize, chance: f64) {
        let mut rng = rand::rng();
        let best_n = 5;
        let max_genomes: usize = self.population.len();

        for _ in 0..iterations {
            let mut new_population = Vec::with_capacity(max_genomes);

            let best_genomes = self.best_genomes(best_n);
            new_population.extend(best_genomes.clone());

            while new_population.len() < max_genomes {
                let idx = self.population.len() % best_n;
                let mut genome = best_genomes[idx].clone();
                genome.mutate(&mut rng, chance);
                new_population.push(genome);
            }

            self.population = new_population
                .iter()
                .map(|genome| genome.build(&mut rng))
                .collect();
        }
    }

    pub fn print_best_n(&self, n: usize) {
        let best_genomes = self.best_genomes(n);
        for genome in best_genomes.iter() {
            println!("Best Genome {:?}", genome.fitness());
        }
    }
}

fn main() {
    let mut rng = rand::rng();

    let options = Options { min: 0, max: 1 };

    let genes: Vec<Gene> = (0..30)
        .map(|i| Gene::new(&format!("gene{}", i + 1), options))
        .collect();

    let genomes =
        (0..50).map(|_| Genome::new(vec![Chromosome::new(genes.clone())]).build(&mut rng));

    let mut population = Population::new(genomes.collect());
    population.print_best_n(5);
    println!("---");

    population.run(100, 0.1);

    population.print_best_n(5);

    // println!(":?", genome.clone());
}
