use num_traits::PrimInt;

/// list all subset combinations of `n` element in given set of `r` element.
/// This is a recursive function that collects all subsets of the set of size
/// `n` with the given set of size `r`.
pub fn list_subset<T: PrimInt>(
    set: &[T],
    n: usize,
    r: usize,
    index: usize,
    data: &mut [T],
    i: usize,
) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity(n * r);

    // Current subset is ready to be added to the list
    if i == r {
        res.push(data[0..r].to_vec());
        return res;
    }

    // More elements are there to put in data
    if n > index {
        // current included, put next at next location
        data[i] = set[index];
        res.append(&mut list_subset(set, n, r, index + 1, data, i + 1));
        // current excluded, replaced with next (i+1 is passed, index unchanged)
        res.append(&mut list_subset(set, n, r, index + 1, data, i));
    }
    res
}

/// list all subset combinations of `N` element in given set of `R` element.
/// This is a recursive function that collects all subsets of the set of size
/// `N` with the given set of size `R`.
#[inline]
pub fn subsets<T: PrimInt, const N: usize, const R: usize>(
    set: &[T],
    index: usize,
    data: &mut [T],
    i: usize,
) -> Vec<Vec<T>> {
    // Current subset is ready to be added to the list
    if i == R {
        let res = vec![data[0..R].to_vec()];
        return res;
    }

    let mut res = Vec::with_capacity(N * R);

    // More elements are there to put in data
    if N > index {
        // current included, put next at next location
        data[i] = set[index];
        res.append(&mut subsets::<T, N, R>(set, index + 1, data, i + 1));
        // current excluded, replaced with next (i+1 is passed, index unchanged)
        res.append(&mut subsets::<T, N, R>(set, index + 1, data, i));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_const_susbset3() {
        let res = subsets::<isize, 5, 3>(&[1, 2, 3, 4, 5], 0, &mut [0; 3], 0);

        assert_eq!(res, vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5]
        ]);
    }

    #[test]
    fn test_capacity() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 3;
        let mut data = [0; R];
        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(res.capacity(), n * R);
    }

    #[test]
    fn test_print_subset3() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 3;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(res, vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5]
        ]);
    }

    #[test]
    fn test_print_subset4() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 4;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(res, vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 5],
            vec![1, 2, 4, 5],
            vec![1, 3, 4, 5],
            vec![2, 3, 4, 5]
        ]);
    }

    #[test]
    fn test_print_subset5() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 5;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(res, vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn test_print_incorrect_subset() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 6;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        let result_set: Vec<Vec<i32>> = Vec::new();
        assert_eq!(res, result_set);
    }
}
