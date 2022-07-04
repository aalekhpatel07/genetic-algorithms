use std::time::Instant;


#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            verbose: false,
        }
    }
}

pub trait Evolution {
    type Member;
    type Gene;
    const MAX_FITNESS: f64 = 1.0;

    fn generate_member(&self) -> Self::Member;
    fn get_fitness(&self, member: &Self::Member) -> f64;
    fn mutate(&self, member: &Self::Member) -> Self::Member;

    fn display(&self, member: &Self::Member, start_time: std::time::Instant) 
        where Self::Member: std::fmt::Display
    {
        let time_difference = Instant::now() - start_time;
        let fitness = self.get_fitness(member);
        println!("{}\t{:.4}\t{:?}", member, fitness, time_difference);
    }

    fn evolve(&self) -> (Self::Member, f64)
        where Self::Member: std::fmt::Display
    {
        self.evolve_with_config(Config::default())
    }

    fn evolve_with_config(&self, config: Config) -> (Self::Member, f64)
        where Self::Member: std::fmt::Display
    {
        let start_time = Instant::now();

        let mut best_member = self.generate_member();
        let mut best_fitness = self.get_fitness(&best_member);

        if config.verbose {
            self.display(&best_member, start_time);
        }

        loop {
            let child_member = self.mutate(&best_member);
            let child_fitness = self.get_fitness(&child_member);


            if best_fitness >= child_fitness {
                continue;
            }

            if config.verbose {
                self.display(&child_member, start_time);
            }
            
            best_fitness = child_fitness;
            best_member = child_member;

            if child_fitness >= Self::MAX_FITNESS {
                break;
            }
        }
        (best_member, best_fitness)
    }

}
