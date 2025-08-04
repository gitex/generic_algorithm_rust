use crate::structs::{Build, Gene};

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<Gene>,
}

impl Chromosome {
    pub fn new(genes: Vec<Gene>) -> Self {
        Chromosome { genes }
    }

    pub fn iter(&self) -> impl Iterator<Item = Gene> {
        self.genes.iter().cloned()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Gene> {
        self.genes.iter_mut()
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
