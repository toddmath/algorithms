//! # Bead sort algorithm
//!
//! Only works for sequences of non-negative integers.
//! [Wikimedia](https://en.wikipedia.org/wiki/Bead_sort)

use num_integer::Integer;

/// Bead sort. Only works for sequences of non-negative integers.
///
/// [Wikimedia](https://en.wikipedia.org/wiki/Bead_sort)
pub fn bead_sort<T: Integer + TryInto<usize> + TryFrom<usize> + Copy>(data: &mut [T]) {
    let max = data
        .iter()
        .max()
        .or(data.first())
        .copied()
        .unwrap()
        .try_into()
        .ok()
        .unwrap();
    let mut beads = vec![vec![0; max]; data.len()];

    for i in 0..data.len() {
        for j in (0..data[i].try_into().ok().unwrap()).rev() {
            beads[i][j] = 1;
        }
    }

    for j in 0..max {
        let sum: usize = beads.iter().take(data.len()).map(|x| x[j]).sum();
        for bead in beads.iter_mut().take(data.len()) {
            bead[j] = 0;
        }
        for k in ((data.len() - sum)..data.len()).rev() {
            let j: T = j.try_into().ok().unwrap();
            data[k] = j + T::one();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn descending() {
        // descending
        let mut ve1: [u64; 5] = [5, 4, 3, 2, 1];
        let cloned = ve1;
        bead_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn mix_values() {
        // pre-sorted
        let mut ve2: [isize; 5] = [7, 9, 6, 2, 3];
        let cloned = ve2;
        bead_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
