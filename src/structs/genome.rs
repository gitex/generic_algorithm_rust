use crate::structs::{Build, Chromosome};

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
    // pub fn fitness<F>(&self, f: F, smooth: f64) -> f64
    // where
    //     F: Fn(&Self) -> f64,
    // {
    //     f(self) / smooth
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
