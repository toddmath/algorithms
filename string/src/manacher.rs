use std::cmp;

use itertools::Itertools;

/// Manacher
pub fn manacher(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    let l = s.len();
    if l <= 1 {
        return s.to_string();
    }

    let mut chars = Itertools::intersperse(s.chars(), '#').collect_vec();
    chars.push('#');

    let mut length_of_palindrome = vec![1usize; chars.len()];
    let (mut current_center, mut right_from_current_center) = (0, 0);

    for i in 0..chars.len() {
        if right_from_current_center > i && i > current_center {
            length_of_palindrome[i] = cmp::min(
                right_from_current_center - i,
                length_of_palindrome[2 * current_center - i],
            );

            if length_of_palindrome[i] + i > right_from_current_center {
                current_center = i;
                right_from_current_center = length_of_palindrome[i] + i;

                if right_from_current_center >= chars.len() - 1 {
                    break;
                }
            } else {
                continue;
            }
        }

        let mut radius = ((length_of_palindrome[i] - 1) / 2) + 1;

        while i >= radius && i + radius <= chars.len() - 1 && chars[i - radius] == chars[i + radius]
        {
            length_of_palindrome[i] += 2;
            radius += 1;
        }
    }

    let center_of_max = length_of_palindrome
        .iter()
        .position_max()
        .expect("No maximum found");
    let radius_of_max = (length_of_palindrome[center_of_max] - 1) / 2;

    chars[center_of_max - radius_of_max..center_of_max + radius_of_max + 1]
        .iter()
        .filter(|&c| c != &'#')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::manacher;

    #[test]
    fn get_longest_palindrome_by_manacher() {
        assert_eq!(manacher("babad"), "aba".to_string());
        assert_eq!(manacher("cbbd"), "bb".to_string());
        assert_eq!(manacher("a"), "a".to_string());

        let ac_ans = manacher("ac");
        assert!(ac_ans == *"a" || ac_ans == *"c");
    }
}
