#[allow(unused_imports)]
use rand::Rng;

mod structs;

use structs::{Build, Chromosome, Gene, Genome, Options};

fn main() {
    let mut rng = rand::rng();

    let genome = Genome::new(vec![
        Chromosome::new(vec![
            Gene::new("gene1", Options { min: 1, max: 10 }),
            Gene::new("gene2", Options { min: 5, max: 15 }),
        ]),
        Chromosome::new(vec![
            Gene::new("gene3", Options { min: 10, max: 20 }),
            Gene::new("gene4", Options { min: 0, max: 5 }),
        ]),
    ]);

    genome.build(&mut rng);
    // println!(":?", genome.clone());
}
