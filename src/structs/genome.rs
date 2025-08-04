use crate::structs::{Build, Chromosome};

pub struct Genome {
    chromosomes: Vec<Chromosome>,
}

impl Genome {
    pub fn new(chromosomes: Vec<Chromosome>) -> Self {
        Genome { chromosomes }
    }
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
