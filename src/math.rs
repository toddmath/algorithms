//! Common math algorithms

mod abs;
mod collatz_sequence;
mod fast_fourier_transform;
mod faster_perfect_numbers;
mod gaussian_elimination;
mod lcm_of_n_number;
mod linear_sieve;
mod matrix_ops;
mod mersenne_primes;
mod prime_numbers;

use self::abs::abs;
use self::collatz_sequence::sequence;
use self::fast_fourier_transform::*;
use self::faster_perfect_numbers::generate_perfect_numbers;
use self::gaussian_elimination::gaussian_elimination;
use self::lcm_of_n_number::lcm;
use self::linear_sieve::LinearSieve;
use self::matrix_ops::*;
use self::mersenne_primes::{get_mersenne_primes, is_mersenne_prime};
use self::prime_numbers::prime_numbers;
