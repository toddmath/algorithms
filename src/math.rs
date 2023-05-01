//! Common math algorithms

mod abs;
mod aliquot_sum;
mod collatz_sequence;
mod factors;
mod fast_fourier_transform;
mod faster_perfect_numbers;
mod gaussian_elimination;
mod lcm_of_n_number;
mod linear_sieve;
mod matrix_ops;
mod mersenne_primes;
mod prime_numbers;
mod random;

pub use self::{
    abs::abs,
    aliquot_sum::aliquot_sum,
    collatz_sequence::sequence,
    factors::factors,
    fast_fourier_transform::*,
    faster_perfect_numbers::generate_perfect_numbers,
    gaussian_elimination::gaussian_elimination,
    lcm_of_n_number::lcm,
    linear_sieve::LinearSieve,
    matrix_ops::*,
    mersenne_primes::{get_mersenne_primes, is_mersenne_prime},
    prime_numbers::prime_numbers,
    random::PCG32,
};
