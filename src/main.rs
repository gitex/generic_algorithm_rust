use std::path::Path;

#[allow(unused_imports)]
use rand::Rng;

mod structs;
use structs::{
    BEST_FITNESS, Build, Chromosome, Fitness, GENERATION, Gene, Genome, LogEntry, LogEvent, Logger,
    Options,
};

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
    generation: u64,
    population: Vec<Genome>,
    logger: Logger<LogEntry>,
}

impl Population {
    pub fn new(population: Vec<Genome>) -> Self {
        Population {
            generation: 1,
            population,
            logger: Logger::new(),
        }
    }

    pub fn best_genomes(&self, count: usize) -> Vec<Genome> {
        let mut genomes = self.population.clone();
        genomes.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        genomes.into_iter().take(count).collect()
    }

    pub fn best_fitness(&self) -> f64 {
        self.best_genomes(1)
            .first()
            .map_or(0.0, |genome| genome.fitness())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Genome> {
        self.population.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Genome> {
        self.population.iter_mut()
    }

    pub fn run(&mut self, generations: usize, chance: f64) {
        let mut rng = rand::rng();

        let best_n = 5;
        let max_genomes: usize = self.population.len();
        let mut best_fitness: f64 = 0.0;

        for _ in 0..generations {
            let mut new_population = Vec::with_capacity(max_genomes);

            // Get Elite genomes
            let mut best_genomes = self.best_genomes(best_n);
            new_population.extend(best_genomes.clone());

            for genome in best_genomes.iter() {
                self.logger.add_entry(LogEntry::new(
                    self.generation,
                    genome.id,
                    genome.parents,
                    LogEvent::Elite,
                    format!("{}", genome),
                    genome.fitness(),
                    best_fitness,
                ))
            }

            // Save best fitness for the generation
            best_fitness = self.best_fitness();

            while new_population.len() < max_genomes {
                let idx = self.population.len() % best_n;
                let mut genome = best_genomes[idx].clone();
                genome.mutate(&mut rng, chance);
                new_population.push(genome);
            }

            self.generation += 1;
            self.population = new_population
                .iter()
                .map(|genome| genome.build(&mut rng))
                .collect();
        }
    }

    pub fn dump(&self, path: &Path) {
        self.logger.save_to_file(path);
    }
}

fn main() {
    let mut rng = rand::rng();

    let options = Options { min: 0, max: 1 };

    let genes: Vec<Gene> = (0..30)
        .map(|i| Gene::new(&format!("gene{}", i + 1), options))
        .collect();

    let genomes = (0..50)
        .map(|_| Genome::new(vec![Chromosome::new(genes.clone())], Some([0, 0])).build(&mut rng));

    let mut population = Population::new(genomes.collect());

    population.run(100, 0.1);
    population.dump(Path::new("log.json"));
}
