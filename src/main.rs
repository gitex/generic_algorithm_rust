use rand::Rng;

pub enum Gene {}
pub struct Chromosome(Vec<Gene>);
pub struct Genome(Vec<Chromosome>);

impl Genome {
    pub fn fitness(&self) {}
    pub fn mutate(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}
