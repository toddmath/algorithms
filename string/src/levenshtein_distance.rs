use itertools::Itertools;

/// The Levenshtein distance (or edit distance) between 2 strings.
/// This edit distance is defined as being 1 point per insertion, substitution,
/// or deletion which must be made to make the strings equal. This function
/// iterates over the bytes in the string, so it may not behave entirely as
/// expected for non-ASCII strings.
///
/// For a detailed explanation, check the example on [Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance)
/// (see the examples with the matrices, for instance between KITTEN and
/// SITTING)
///
/// Note that although we compute a matrix, left-to-right, top-to-bottom, at
/// each step all we need to compute `cell[i][j]` is:
///   - `cell[i][j-1]`
///   - `cell[i-j][j]`
///   - `cell[i-i][j-1]`
///
/// This can be achieved by only using one "rolling" row and one additional
/// variable, when computed `cell[i][j]` (or `row[i]`):
///   - `cell[i][j-1]` is the value to the left, on the same row (the one we
///     just computed, `row[i-1]`)
///   - `cell[i-1][j]` is the value at `row[i]`, the one we're changing
///   - `cell[i-1][j-1]` was the value at `row[i-1]` before we changed it, for
///     that we'll use a variable
///
/// Doing this reduces space complexity from O(nm) to O(n)
///
/// Second note: if we want to minimize space, since we're now O(n) make sure
/// you use the shortest string horizontally, and the longest vertically
///
/// # Complexity
///   - time complexity: O(nm),
///   - space complexity: O(n),
///
/// where n and m are lengths of `a` and `b`
pub fn levenshtein_distance(a: impl AsRef<str>, b: impl AsRef<str>) -> usize {
    let (a, b) = (a.as_ref(), b.as_ref());
    if a.is_empty() {
        return b.len();
    }
    let na = a.len();
    let mut prev_dist = (0..=na).collect_vec();

    for (row, c2) in b.char_indices() {
        let mut prev_substitution_cost = prev_dist[0];
        prev_dist[0] = row + 1;

        for (col, c1) in a.char_indices() {
            let deletion_cost = prev_dist[col] + 1;
            let insertion_cost = prev_dist[col + 1] + 1;
            let substitution_cost = prev_substitution_cost + (c1 != c2) as usize;
            prev_substitution_cost = prev_dist[col + 1];
            prev_dist[col + 1] = deletion_cost.min(insertion_cost.min(substitution_cost));
        }
    }

    prev_dist[na]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_example() {
        assert_eq!(2, levenshtein_distance("FROG", "DOG"));
    }

    #[test]
    fn empty_strings() {
        assert_eq!(0, levenshtein_distance("", ""));
    }

    #[test]
    fn basic() {
        assert_eq!(1, levenshtein_distance("", "a"));
        assert_eq!(1, levenshtein_distance("a", ""));
        assert_eq!(1, levenshtein_distance("ab", "a"));
        assert_eq!(0, levenshtein_distance("foobar", "foobar"));
        assert_eq!(6, levenshtein_distance("foobar", "barfoo"));
        assert_eq!(1, levenshtein_distance("kind", "bind"));
        assert_eq!(3, levenshtein_distance("winner", "win"));
    }

    #[test]
    fn equal_strings() {
        assert_eq!(0, levenshtein_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, levenshtein_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, levenshtein_distance("Test_Case_#1", "Test_Case_#1"));
        assert_eq!(0, levenshtein_distance("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, levenshtein_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#10"));
        assert_eq!(1, levenshtein_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, levenshtein_distance("My Cat", "My Case"));
        assert_eq!(7, levenshtein_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, levenshtein_distance("Test_Case_#3", "Case #3"));
        assert_eq!(2, levenshtein_distance("My Cat", "My Case"));
        assert_eq!(7, levenshtein_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, levenshtein_distance("Test_Case_#3", "Case #3"));
    }
}
