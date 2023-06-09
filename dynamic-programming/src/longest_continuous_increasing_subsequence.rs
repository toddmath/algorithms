/// Longest continuous increasing subsequence of `data`.
pub fn longest_continuous_increasing_subsequence<T: Ord + Clone>(data: &[T]) -> &[T] {
    let length = data.len();
    if length <= 1 {
        return data;
    }

    let mut tracking = vec![1; length];
    for i in (0..length - 1).rev() {
        if data[i] < data[i + 1] {
            tracking[i] = tracking[i + 1] + 1;
        }
    }

    let (mut max_index, mut max_value) = (0, 0);
    for (index, &value) in tracking.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = index;
        }
    }

    // &data[max_index..max_index + max_value as usize]

    // SAFETY: we have to be in bounds with these indexes
    unsafe { data.get_unchecked(max_index..max_index + max_value as usize) }
}

#[cfg(test)]
mod tests {
    use super::longest_continuous_increasing_subsequence;

    #[test]
    fn test_longest_increasing_subsequence() {
        // Base Cases
        let base_case_array: [i32; 0] = [];
        assert_eq!(
            &longest_continuous_increasing_subsequence(&base_case_array),
            &[]
        );
        assert_eq!(&longest_continuous_increasing_subsequence(&[1]), &[1]);

        // Normal i32 Cases
        assert_eq!(
            &longest_continuous_increasing_subsequence(&[1, 2, 3, 4]),
            &[1, 2, 3, 4]
        );
        assert_eq!(
            &longest_continuous_increasing_subsequence(&[1, 2, 2, 3, 4, 2]),
            &[2, 3, 4]
        );
        assert_eq!(
            &longest_continuous_increasing_subsequence(&[5, 4, 3, 2, 1]),
            &[5]
        );
        assert_eq!(
            &longest_continuous_increasing_subsequence(&[5, 4, 3, 4, 2, 1]),
            &[3, 4]
        );

        // Non-Numeric case
        assert_eq!(
            &longest_continuous_increasing_subsequence(&['a', 'b', 'c']),
            &['a', 'b', 'c']
        );
        assert_eq!(
            &longest_continuous_increasing_subsequence(&['d', 'c', 'd']),
            &['c', 'd']
        );
    }
}
