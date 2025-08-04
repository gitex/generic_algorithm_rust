use rand::Rng;
use rand::distr::uniform::SampleUniform;
use std::ops::{Range, RangeBounds, RangeInclusive};

pub trait Build {
    fn build<R: Rng + ?Sized>(&self, rng: &mut R) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub struct Options<T> {
    pub min: T,
    pub max: T,
}

#[derive(Clone, Debug)]
pub struct Gene<T = u64> {
    name: String,
    value: Option<T>,
    options: Options<T>,
}

impl<T> Gene<T> {
    pub fn new(name: &str, range: Options<T>) -> Self {
        Gene {
            name: name.to_string(),
            value: None,
            options: range,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl<T: SampleUniform + PartialOrd + Clone + Copy> Build for Gene<T> {
    fn build<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let value = Some(rng.random_range(self.options.min..=self.options.max));

        Self {
            name: self.name.clone(),
            value,
            options: self.options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_u8_gene_creation() {
        let mut rng = rand::rng();
        let gene = Gene::<u8>::new("test_gene", Options { min: 1, max: 10 });
        let built_gene = gene.build(&mut rng);

        assert_eq!(built_gene.get_name(), "test_gene");
        assert!(built_gene.get_value().is_some());
        assert!(built_gene.get_value().unwrap() >= &1 && built_gene.get_value().unwrap() <= &10);
    }

    #[test]
    fn test_f32_gene_creation() {
        let mut rng = rand::rng();
        let gene = Gene::<f32>::new(
            "test_f32_gene",
            Options {
                min: 1.0,
                max: 10.0,
            },
        );
        let built_gene = gene.build(&mut rng);

        assert_eq!(built_gene.get_name(), "test_f32_gene");
        assert!(built_gene.get_value().is_some());
        assert!(
            built_gene.get_value().unwrap() >= &1.0 && built_gene.get_value().unwrap() <= &10.0
        );
    }

    #[test]
    fn test_bit_gene_creation() {
        let mut rng = rand::rng();
        let gene = Gene::<u8>::new("test_bit_gene", Options { min: 0, max: 1 });
        let built_gene = gene.build(&mut rng);
        assert_eq!(built_gene.get_name(), "test_bit_gene");
        assert!(built_gene.get_value().is_some());
        assert!(built_gene.get_value().unwrap() == &0 || built_gene.get_value().unwrap() == &1);
    }
}
