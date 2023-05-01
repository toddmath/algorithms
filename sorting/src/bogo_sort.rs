use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::PCG32;

const DEFAULT: u64 = 4294967296;

#[cfg(target_pointer_width = "64")]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u64() as usize % range
}

#[cfg(not(target_pointer_width = "64"))]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u32() as usize % range
}

/// Fisher–Yates shuffle for generating random permutation.
fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
    for i in (1..len).rev() {
        let j = generate_index(i + 1, generator);
        arr.swap(i, j);
    }
}

/// Bogo sort
pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(DEFAULT, |d| d.as_millis() as u64);

    let mut random_generator = PCG32::new_default(seed);
    let len = arr.len();

    while !arr.is_sorted() {
        permute_randomly(arr, len, &mut random_generator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_array() {
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn sorted_array() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
