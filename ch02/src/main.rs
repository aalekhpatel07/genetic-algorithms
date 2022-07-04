use std::{fmt::Display, ops::{Deref, DerefMut}};

use genetic::{Evolution, Config};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct OneMaximizer {
    genes: Genes,
    pub target: usize
}

#[derive(Debug, Clone)]
pub struct Genes(Vec<bool>);

impl Deref for Genes {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Genes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Genes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
        .iter()
        .map(|&x| if x { '1' } else { '0' })
        .collect::<String>();
        write!(f, "{}", s)
    }
}

impl Default for OneMaximizer {
    fn default() -> Self {
        Self::new(100)
    }
}

impl OneMaximizer {
    pub fn new(target: usize) -> Self {
        OneMaximizer {
            genes: Genes(vec![true, false]),
            target
        }
    }
}


impl Evolution for OneMaximizer {
    type Member = Genes;
    type Gene = usize;

    fn generate_member(&self) -> Self::Member {
        let mut rng = thread_rng();

        Genes((0..self.target)
        .map(|_| {
            self.genes[rng.gen_range(0..self.genes.len())]
        }).collect())
    }

    fn get_fitness(&self, member: &Self::Member) -> f64 {
        member.iter().filter(|&x| *x == true).count() as f64 / self.target as f64
    }

    fn mutate(&self, member: &Self::Member) -> Self::Member {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..self.target);

        let mut new_member = member.clone();

        *new_member.get_mut(idx).unwrap() = !new_member[idx];

        new_member
    }


}
pub fn main() {
    let guesser = OneMaximizer::new(10000);
    let (_, best_fitness) = guesser.evolve();
    println!("{}", best_fitness);
}


#[cfg(test)]
mod tests {
    use super::{OneMaximizer, Evolution};

    #[test]
    fn guess_size_10() {
        let target = 10;
        let guesser = OneMaximizer::new(target);
        let (best_guess, _) = guesser.evolve();
        assert_eq!(target, best_guess.iter().filter(|&x| *x == true).count());
    }

    #[test]
    fn guess_size_1000() {
        let target = 1000;
        let guesser = OneMaximizer::new(target);
        let (best_guess, _) = guesser.evolve();
        assert_eq!(target, best_guess.iter().filter(|&x| *x == true).count());
    }
}
