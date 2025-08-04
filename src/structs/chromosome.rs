use crate::structs::{Build, Gene};

pub struct Chromosome {
    genes: Vec<Gene>,
}

impl Chromosome {
    pub fn new(genes: Vec<Gene>) -> Self {
        Chromosome { genes }
    }
}

impl Build for Chromosome {
    fn build<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut genes = Vec::with_capacity(self.genes.len());
        for gene in &self.genes {
            genes.push(gene.build(rng));
        }
        Chromosome { genes }
    }
}
