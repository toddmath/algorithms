/// Pantience sort
pub fn patience_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    if arr.is_empty() {
        return;
    }
    let mut piles: Vec<Vec<T>> = Vec::with_capacity(arr.len());

    for &card in arr.iter() {
        let mut left = 0;
        let mut right = piles.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if *piles[mid].last().unwrap() < card {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == piles.len() {
            piles.push(vec![card]);
        } else {
            piles[left].push(card);
        }
    }

    let mut idx = 0;
    while let Some((min_id, pile)) = piles
        .iter()
        .enumerate()
        .min_by_key(|&(_, pile)| *pile.last().unwrap())
    {
        arr[idx] = *pile.last().unwrap();
        idx += 1;
        piles[min_id].pop();

        if piles[min_id].is_empty() {
            _ = piles.remove(min_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut array: Vec<isize> = vec![
            -2, 7, 15, -14, 0, 15, 0, 10_033, 7, -7, -4, -13, 5, 8, -14, 12,
        ];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn empty() {
        let mut array = Vec::<i32>::new();
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn one_element() {
        let mut array = vec![3];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut array = vec![-123_456, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }
}
