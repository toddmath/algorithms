/// Bead sort. Only works for sequences of non-negative integers.
///
/// - [Wiki](https://en.wikipedia.org/wiki/Bead_sort)
pub fn bead_sort(data: &mut [usize]) {
    let max = data.iter().max().copied().unwrap_or(data[0]);
    let mut beads = vec![vec![0; max]; data.len()];

    for i in 0..data.len() {
        for j in (0..data[i]).rev() {
            beads[i][j] = 1;
        }
    }

    for j in 0..max {
        let sum: usize = beads.iter().take(data.len()).map(|x| x[j]).sum();
        for bead in beads.iter_mut().take(data.len()) {
            bead[j] = 0;
        }
        for k in ((data.len() - sum)..data.len()).rev() {
            data[k] = j + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::{have_same_elements, is_sorted};

    #[test]
    fn descending() {
        // descending
        let mut ve1: [usize; 5] = [5, 4, 3, 2, 1];
        let cloned = ve1;
        bead_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn mix_values() {
        // pre-sorted
        let mut ve2: [usize; 5] = [7, 9, 6, 2, 3];
        let cloned = ve2;
        bead_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
