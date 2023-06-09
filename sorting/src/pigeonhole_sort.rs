use itertools::{Itertools, MinMaxResult::MinMax};
use num_traits::{NumAssignOps, PrimInt, ToPrimitive};

/// Pigeonhole sort
pub fn pigeonhole_sort<T>(arr: &mut [T])
where
    T: NumAssignOps + PrimInt + ToPrimitive,
{
    if let MinMax(&min, &max) = arr.iter().minmax() {
        let n_holes = (max - min + T::one()).to_usize().unwrap();
        let mut holes = vec![T::zero(); n_holes];
        let mut holes_repeat = vec![T::zero(); n_holes];

        for i in arr.iter() {
            let index = (*i - min).to_usize().unwrap();
            holes[index] = *i;
            holes_repeat[index] += T::one();
        }

        let mut index = 0;
        for i in 0..n_holes {
            while !holes_repeat[i].is_zero() {
                arr[index] = holes[i];
                index += 1;
                holes_repeat[i] -= T::one();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn test1() {
        let mut arr1 = [3, 3, 3, 1, 2, 6, 5, 5, 5, 4, 1, 6, 3];
        pigeonhole_sort(&mut arr1);
        assert!(is_sorted(&arr1));

        let mut arr2 = [6usize, 5, 4, 3, 2, 1];
        pigeonhole_sort(&mut arr2);
        assert!(is_sorted(&arr2));
    }
}
