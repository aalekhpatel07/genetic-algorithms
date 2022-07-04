use genetic::Evolution;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct StringGuesser {
    genes: Vec<char>,
    pub target: String
}

impl StringGuesser {
    pub fn new(target: &str) -> Self {
        StringGuesser { 
            target: String::from(target),
            genes: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[{]};:'\",<.>/?|\\`~ ".chars().into_iter().collect::<Vec<_>>()
        }
    }
    pub fn with_genes(self: Self, genes: &[char]) -> Self {
        StringGuesser { 
            target: self.target,
            genes: genes.to_vec()
        }
    }
    pub fn with_target(self: Self, target: &str) -> Self {
        StringGuesser { 
            target: String::from(target),
            genes: self.genes
        }
    }
}

impl Default for StringGuesser {
    fn default() -> Self {
        StringGuesser::new("")
    }
}

impl Evolution for StringGuesser {
    type Member = String;
    type Gene = char;

    fn generate_member(&self) -> Self::Member {
        let mut rng = thread_rng();
        (0..self.target.len())
        .map(|_| {
            let idx = rng.gen_range(0..self.genes.len());
            *self.genes.get(idx).unwrap() as char
        }).collect()
    }

    fn get_fitness(&self, member: &Self::Member) -> f64 {
        self
        .target
        .chars()
        .zip(member.chars())
        .filter(|&(a, b)| a == b)
        .count() as f64 / self.target.len() as f64
    }
    fn mutate(&self, member: &Self::Member) -> Self::Member {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..member.len());

        let mut new_member = member.clone();

        let new_gene = *self.genes.get(rng.gen_range(0..self.genes.len())).unwrap();
        let alternate_gene = *self.genes.get(rng.gen_range(0..self.genes.len())).unwrap();

        let previous_gene = new_member.chars().nth(idx).unwrap();
        if previous_gene == new_gene {
            new_member.replace_range(idx..idx+1, &String::from(alternate_gene));
        } else {
            new_member.replace_range(idx..idx+1, &String::from(new_gene));
        }
        new_member
    }


}
pub fn main() {
    let guesser = StringGuesser::new("Hello, world!");
    let (_, best_fitness) = guesser.evolve();
    println!("{}", best_fitness);
}


#[cfg(test)]
mod tests {
    use super::{StringGuesser, Evolution};

    #[test]
    fn guess_hello_world() {
        let target = "Hello World!";
        let guesser = StringGuesser::new(target);
        let (best_guess, _) = guesser.evolve();
        assert_eq!(target, best_guess);
    }

    #[test]
    fn guess_a_really_long_string() {
        let target = "This is a really long string that is going to be used to test the evolution algorithm.";
        let guesser = StringGuesser::new(target);
        let (best_guess, _) = guesser.evolve();
        assert_eq!(target, best_guess);
    }
}
