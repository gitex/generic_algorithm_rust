use crate::structs::{Build, Chromosome, GENERATION, GENOME_ID_GENERATOR};
use rand::Rng;
use std::fmt;

pub trait Fitness {
    fn fitness(&self) -> f64;
}

pub type GenomeId = usize;
pub type Parents = [usize; 2];

#[derive(Clone)]
pub struct Genome {
    pub id: GenomeId,
    pub parents: Parents,
    pub chromosomes: Vec<Chromosome>,
}

impl Genome {
    pub fn new(chromosomes: Vec<Chromosome>, parents: Option<Parents>) -> Self {
        let parents = parents.unwrap_or([0, 0]);

        Genome {
            id: GENOME_ID_GENERATOR.next(),
            parents,
            chromosomes,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Chromosome> {
        self.chromosomes.iter().cloned()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Chromosome> {
        self.chromosomes.iter_mut()
    }

    pub fn mutate<R: Rng + ?Sized>(&mut self, rng: &mut R, chance: f64) {
        for chromosome in self.iter_mut() {
            for gene in chromosome.iter_mut() {
                if rng.random::<f64>() <= chance {
                    gene.mutate(rng);
                }
            }
        }
    }
}

impl Build for Genome {
    fn build<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut chromosomes = Vec::with_capacity(self.chromosomes.len());

        for chromosome in &self.chromosomes {
            chromosomes.push(chromosome.build(rng));
        }

        Genome::new(chromosomes, None)
    }
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut genes = String::new();

        genes.push_str("| ");

        for chromosome in self.iter() {
            for gene in chromosome.iter() {
                let value = gene.value();

                if value.is_none() {
                    genes.push_str("None ");
                    continue;
                }

                genes.push_str(&format!("{} ", value.unwrap()));
            }

            genes.push_str("| ");
        }

        write!(f, "{}", genes)
    }
}
