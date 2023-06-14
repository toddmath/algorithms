use num_traits::PrimInt;

/// Hanoi tower solution.
pub fn hanoi<T: PrimInt>(n: T, from: T, to: T, via: T, moves: &mut Vec<(T, T)>) {
    if n > T::zero() {
        hanoi(n - T::one(), from, via, to, moves);
        moves.push((from, to));
        hanoi(n - T::one(), via, to, from, moves);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hanoi_simple() {
        let correct_solution = vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)];
        let mut our_solution = Vec::new();
        hanoi(3, 1, 3, 2, &mut our_solution);
        assert_eq!(correct_solution, our_solution);
    }
}
