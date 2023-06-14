//! The goal is to showcase how Genetic algorithms generically work
//! See: https://en.wikipedia.org/wiki/Genetic_algorithm for concepts

use std::{array, cmp::Ordering, collections::BTreeSet, fmt::Debug};

use itertools::{Either, Itertools};
use rand::Rng;
// use num_traits::{Float, PrimInt, Unsigned};

/// This is the definition of a Chromosome for a genetic algorithm
/// We can picture this as "one contending solution to our problem"
/// It is generic over:
/// * Eval, which could be a float, or any other totally ordered type, so that
///   we can rank solutions to our problem
/// * R: a random number generator (could be thread rng, etc.)
pub trait Chromosome<R: Rng, Eval> {
    /// Mutates the chromosome, changing it's genes.
    fn mutate(&mut self, rng: &mut R);

    /// Mixes this chromosome with another one.
    fn crossover(&self, other: &Self, rng: &mut R) -> Self;

    /// How well this chromosome fits the problem we're trying to solve.
    /// **The smaller the better it fits**. For instance, we could use
    /// abs(... - expected_value).
    fn fitness(&self) -> Eval;
}

/// Selection strategy for a genetic algorithm.
pub trait SelectionStrategy<R: Rng> {
    /// Create a new [`SelectionStrategy`] with random number generator `rng`.
    fn new(rng: R) -> Self;

    /// Selects a portion of the population for reproduction
    /// Could be totally random ones or the ones that fit best, etc.
    /// This assumes the population is sorted by how it fits the solution,
    /// the first the better.
    fn select<'a, Eval: Into<f64>, C: Chromosome<R, Eval>>(
        &mut self,
        population: &'a [C],
    ) -> (&'a C, &'a C);
}

/// Roulette wheel selection strategy.
#[derive(Debug)]
pub struct RouletteWheel<R: Rng> {
    rng: R,
}

impl<R: Rng> SelectionStrategy<R> for RouletteWheel<R> {
    fn new(rng: R) -> Self {
        Self { rng }
    }

    fn select<'a, Eval: Into<f64>, C: Chromosome<R, Eval>>(
        &mut self,
        population: &'a [C],
    ) -> (&'a C, &'a C) {
        // let mut parents = Vec::with_capacity(2);

        let (mut parents, fitnesses): (Vec<_>, Vec<_>) =
            population.iter().partition_map(|individual| {
                let fitness: f64 = individual.fitness().into();
                if individual.fitness().into() == 0.0 {
                    Either::Left(individual)
                } else {
                    Either::Right(1.0 / fitness)
                }
            });

        // let fitnesses = population
        //     .iter()
        //     .filter_map(|individual| {
        //         let fitness: f64 = individual.fitness().into();
        //         if individual.fitness().into() == 0.0 {
        //             parents.push(individual);
        //             None
        //         } else {
        //             Some(1.0 / fitness)
        //         }
        //     })
        //     .collect_vec();

        if parents.len() == 2 {
            return (parents[0], parents[1]);
        }
        let sum: f64 = fitnesses.iter().sum();
        let mut spin = self.rng.gen_range(0.0..=sum);

        for individual in population {
            let fitness = individual.fitness().into();
            if spin <= fitness {
                parents.push(individual);
                if parents.len() == 2 {
                    return (parents[0], parents[1]);
                }
            } else {
                spin -= fitness;
            }
        }

        panic!("Could not select parents");
    }
}

/// Tournament selection strategy.
#[derive(Debug)]
pub struct Tournament<const K: usize, R: Rng> {
    rng: R,
}

impl<const K: usize, R: Rng> SelectionStrategy<R> for Tournament<K, R> {
    fn new(rng: R) -> Self {
        Self { rng }
    }

    fn select<'a, Eval: Into<f64>, C: Chromosome<R, Eval>>(
        &mut self,
        population: &'a [C],
    ) -> (&'a C, &'a C) {
        assert!(K > 2, "K must be > 2");
        let mut index = BTreeSet::from(array::from_fn::<usize, K, _>(|_| {
            self.rng.gen_range(0..population.len())
        }))
        .into_iter();

        // SAFETY: we know that K > 2, so we can unwrap twice
        unsafe {
            (
                &population[index.next().unwrap_unchecked()],
                &population[index.next().unwrap_unchecked()],
            )
        }
    }
}

type Comparator<T> = Box<dyn FnMut(&T, &T) -> Ordering>;

/// A genetic algorithm.
#[allow(missing_debug_implementations)]
pub struct GeneticAlgorithm<R, Eval, C, Selection>
where
    R: Rng,
    Eval: PartialOrd,
    C: Chromosome<R, Eval>,
    Selection: SelectionStrategy<R>,
{
    rng: R,
    population: Vec<C>,
    threshold: Eval,
    max_generations: usize,
    mutation_chance: f64,
    crossover_chance: f64,
    compare: Comparator<Eval>,
    selection: Selection,
}

/// Parameters for a [`GeneticAlgorithm`].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GenericAlgorithmParams {
    max_generations: usize,
    mutation_chance: f64,
    crossover_chance: f64,
}

impl<R, Eval, C, Selection> GeneticAlgorithm<R, Eval, C, Selection>
where
    R: Rng,
    Eval: Into<f64> + PartialOrd + Clone + Debug,
    C: Chromosome<R, Eval> + Clone + Debug,
    Selection: SelectionStrategy<R>,
{
    /// Initialize a new [`GeneticAlgorithm`] with the given parameters.
    #[inline]
    pub fn init(
        rng: R,
        population: Vec<C>,
        threshold: Eval,
        params: GenericAlgorithmParams,
        compare: Comparator<Eval>,
        selection: Selection,
    ) -> Self {
        let GenericAlgorithmParams {
            max_generations,
            mutation_chance,
            crossover_chance,
        } = params;
        Self {
            rng,
            population,
            threshold,
            max_generations,
            mutation_chance,
            crossover_chance,
            compare,
            selection,
        }
    }

    /// Returns the solve of this [`GeneticAlgorithm<R, Eval, C, Selection>`].
    pub fn solve(&mut self) -> Option<C> {
        // let mut generations = 1;
        // while generations <= self.max_generations {
        for _ in 1..=self.max_generations {
            self.population
                .sort_unstable_by(|c1, c2| (self.compare)(&c1.fitness(), &c2.fitness()));

            if let Some(solution) = self.population.first() {
                if solution.fitness() <= self.threshold {
                    return Some(solution).cloned();
                }
            }
            for chromosome in self.population.iter_mut() {
                if self.rng.gen::<f64>() <= self.mutation_chance {
                    chromosome.mutate(&mut self.rng);
                }
            }

            let total_population = self.population.len();

            self.population = {
                let mut res = Vec::with_capacity(total_population + 1);
                while res.len() < total_population {
                    let (p1, p2) = self.selection.select(&self.population);
                    if self.rng.gen::<f64>() <= self.crossover_chance {
                        res.push(p1.crossover(p2, &mut self.rng));
                    } else if res.len() == total_population {
                        res.push(p1.clone());
                    } else {
                        res.extend([p1.clone(), p2.clone()]);
                    }
                }
                res
            };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fmt::{Debug, Formatter},
        ops::RangeInclusive,
    };

    use rand::{rngs::ThreadRng, thread_rng, Rng};

    use super::{
        Chromosome, GenericAlgorithmParams, GeneticAlgorithm, RouletteWheel, SelectionStrategy,
        Tournament,
    };

    #[test]
    // #[ignore] // Too long and not deterministic enough to be part of CI, more of
    // an example than a test
    fn find_secret() {
        let chars = 'a'..='z';
        // cSpell:disable
        let secret = "thisistopsecret".to_owned();
        // Note: we'll pick genes (a, b, c) in the range -10, 10
        // cSpell:enable

        #[derive(Clone)]
        struct TestString {
            chars: RangeInclusive<char>,
            secret: String,
            genes: Vec<char>,
        }

        impl TestString {
            fn new(rng: &mut ThreadRng, secret: String, chars: RangeInclusive<char>) -> Self {
                let current = (0..secret.len())
                    .map(|_| rng.gen_range(chars.clone()))
                    .collect::<Vec<_>>();

                Self {
                    chars,
                    secret,
                    genes: current,
                }
            }
        }

        impl Debug for TestString {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.genes.iter().collect::<String>())
            }
        }

        impl Chromosome<ThreadRng, i32> for TestString {
            fn mutate(&mut self, rng: &mut ThreadRng) {
                // let's assume mutations happen completely randomly, one "gene" at a time (i.e.
                // one char at a time)
                let gene_idx = rng.gen_range(0..self.secret.len());
                let new_char = rng.gen_range(self.chars.clone());
                self.genes[gene_idx] = new_char;
            }

            fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
                let genes = (0..self.secret.len())
                    .map(|idx| {
                        if rng.gen_bool(0.5) {
                            self.genes[idx]
                        } else {
                            other.genes[idx]
                        }
                    })
                    .collect();

                Self {
                    chars: self.chars.clone(),
                    secret: self.secret.clone(),
                    genes,
                }
            }

            fn fitness(&self) -> i32 {
                // We are just counting how many chars are distinct from secret
                self.genes
                    .iter()
                    .zip(self.secret.chars())
                    .filter(|(char, expected)| expected != *char)
                    .count() as i32
            }
        }

        let mut rng = thread_rng();
        let pop_count = 1_000;
        let mut population = Vec::with_capacity(pop_count);
        for _ in 0..pop_count {
            population.push(TestString::new(&mut rng, secret.clone(), chars.clone()));
        }
        let selection: Tournament<100, ThreadRng> = Tournament::new(rng.clone());
        let params = GenericAlgorithmParams {
            max_generations: 100,
            mutation_chance: 0.2,
            crossover_chance: 0.4,
        };
        let mut solver =
            GeneticAlgorithm::init(rng, population, 0, params, Box::new(i32::cmp), selection);
        let res = solver.solve();

        assert!(res.is_some());
        assert_eq!(res.unwrap().genes, secret.chars().collect::<Vec<_>>())
    }

    #[test]
    // #[ignore] // Too long and not deterministic enough to be part of CI, more of
    // an example than a test
    fn solve_mastermind() {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        enum ColoredPeg {
            Red,
            Yellow,
            Green,
            Blue,
            White,
            Black,
        }

        struct GuessAnswer {
            right_pos: i32, // right color at the right pos
            wrong_pos: i32, // right color, but at wrong pos
        }

        #[derive(Clone, Debug)]
        struct CodeMaker {
            // the player coming up with a secret code
            code: [ColoredPeg; 4],
            count_by_color: HashMap<ColoredPeg, usize>,
        }

        impl CodeMaker {
            fn new(code: [ColoredPeg; 4]) -> Self {
                let mut count_by_color = HashMap::with_capacity(4);
                for peg in &code {
                    *count_by_color.entry(*peg).or_default() += 1;
                }
                Self {
                    code,
                    count_by_color,
                }
            }

            fn eval(&self, guess: &[ColoredPeg; 4]) -> GuessAnswer {
                let mut right_pos = 0;
                let mut wrong_pos = 0;
                let mut idx_by_colors = self.count_by_color.clone();

                for (idx, color) in guess.iter().enumerate() {
                    if self.code[idx] == *color {
                        right_pos += 1;
                        let count = idx_by_colors.get_mut(color).unwrap();
                        *count -= 1; // don't reuse to say "right color but wrong pos"
                        if *count == 0 {
                            idx_by_colors.remove(color);
                        }
                    }
                }

                for (idx, color) in guess.iter().enumerate() {
                    if self.code[idx] != *color {
                        // try to use another color
                        if let Some(count) = idx_by_colors.get_mut(color) {
                            *count -= 1;
                            if *count == 0 {
                                idx_by_colors.remove(color);
                            }
                            wrong_pos += 1;
                        }
                    }
                }

                GuessAnswer {
                    right_pos,
                    wrong_pos,
                }
            }
        }

        #[derive(Clone)]
        struct CodeBreaker {
            maker: CodeMaker, // so that we can ask the code maker if our guess is good or not
            guess: [ColoredPeg; 4],
        }

        impl Debug for CodeBreaker {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(format!("{:?}", self.guess).as_str())
            }
        }

        fn random_color(rng: &mut ThreadRng) -> ColoredPeg {
            match rng.gen_range(0..=5) {
                0 => ColoredPeg::Red,
                1 => ColoredPeg::Yellow,
                2 => ColoredPeg::Green,
                3 => ColoredPeg::Blue,
                4 => ColoredPeg::White,
                _ => ColoredPeg::Black,
            }
        }

        fn random_guess(rng: &mut ThreadRng) -> [ColoredPeg; 4] {
            std::array::from_fn(|_| random_color(rng))
        }

        impl Chromosome<ThreadRng, i32> for CodeBreaker {
            fn mutate(&mut self, rng: &mut ThreadRng) {
                // change one random color
                let idx = rng.gen_range(0..4);
                self.guess[idx] = random_color(rng);
            }

            fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
                Self {
                    maker: self.maker.clone(),
                    guess: std::array::from_fn(|i| {
                        if rng.gen::<f64>() < 0.5 {
                            self.guess[i]
                        } else {
                            other.guess[i]
                        }
                    }),
                }
            }

            fn fitness(&self) -> i32 {
                // Ask the code maker for the result
                let answer = self.maker.eval(&self.guess);
                // Remember: we need to have fitness return 0 if the guess is good, and the
                // higher number we return, the further we are from a proper solution
                let mut res = 32; // worst case scenario, everything is wrong
                res -= answer.right_pos * 8; // count 8 points for the right item at the right spot
                res -= answer.wrong_pos; // count 1 point for having a right color
                res
            }
        }

        let code = [
            ColoredPeg::Red,
            ColoredPeg::Red,
            ColoredPeg::White,
            ColoredPeg::Blue,
        ];
        let maker = CodeMaker::new(code);
        let population_count = 10;
        let params = GenericAlgorithmParams {
            max_generations: 100,
            mutation_chance: 0.5,
            crossover_chance: 0.3,
        };
        let mut rng = thread_rng();
        let mut initial_pop = Vec::with_capacity(population_count);
        for _ in 0..population_count {
            initial_pop.push(CodeBreaker {
                maker: maker.clone(),
                guess: random_guess(&mut rng),
            });
        }
        let selection = RouletteWheel { rng: rng.clone() };
        let mut solver =
            GeneticAlgorithm::init(rng, initial_pop, 0, params, Box::new(i32::cmp), selection);
        let res = solver.solve();

        assert!(res.is_some());
        assert_eq!(code, res.unwrap().guess);
    }
}
