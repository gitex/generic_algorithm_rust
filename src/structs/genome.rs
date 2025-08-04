use crate::structs::{Build, Chromosome};
use rand::Rng;

pub trait Fitness {
    fn fitness(&self) -> f64;
}

#[derive(Clone, Debug)]
pub struct Genome {
    chromosomes: Vec<Chromosome>,
}

impl Genome {
    pub fn new(chromosomes: Vec<Chromosome>) -> Self {
        Genome { chromosomes }
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

    // pub fn fitness<F>(&self, f: F, smooth: f64) -> f64
    // where
    //     F: Fn(&Self) -> f64,
    // {
    //     f(self) / smooth
    // }
    //
    // pub fn mutate(&mut self, rng: &mut impl rand::Rng, chance: f64) {
    //     for chromosome in self.chromosomes.iter_mut() {
    //         for gene in chromosome.iter_mut() {
    //             if rng.random::<f64>() < chance {
    //
    //             }
    //         }
    //     }
    // }
}

impl Build for Genome {
    fn build<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut chromosomes = Vec::with_capacity(self.chromosomes.len());
        for chromosome in &self.chromosomes {
            chromosomes.push(chromosome.build(rng));
        }
        Genome { chromosomes }
    }
}
