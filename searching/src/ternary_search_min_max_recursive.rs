use std::cmp::Ordering;

// use itertools::Itertools;
use num_traits::Float;

/// Recursive ternary search algorithm for finding maximum of uni-modal function
pub fn ternary_search_max_rec<T: Float>(
    f: fn(T) -> T,
    start: T,
    end: T,
    // absolute_precision: T,
) -> T {
    // let diff = end.abs_sub(start);
    if (end - start).abs().to_f32().unwrap() > 1e-6 {
        // if (end - start).abs() >= T::epsilon() {
        let (mid1, mid2) = (
            start + (end - start) / T::from(3).unwrap(),
            end - (end - start) / T::from(3).unwrap(),
        );
        let (r1, r2) = (f(mid1), f(mid2));

        match r1.partial_cmp(&r2) {
            Some(Ordering::Less) => {
                return ternary_search_max_rec(f, mid1, end);
            }
            Some(Ordering::Greater) => {
                return ternary_search_max_rec(f, start, mid2);
            }
            Some(Ordering::Equal) => {
                return ternary_search_max_rec(f, mid1, mid2);
            }
            _ => (),
        }
    }
    f(start)
}

/// Recursive ternary search algorithm for finding minimum of uni-modal function
pub fn ternary_search_min_rec<T: Float>(f: fn(T) -> T, start: T, end: T) -> T {
    if (end - start).abs().to_f32().unwrap() >= 1e-6 {
        // if end.abs_sub(start) >= T::epsilon() {
        // let diff = end.abs_sub(start);
        // if diff.is_normal() && diff >= T::epsilon() {
        let (mid1, mid2) = (
            start + (end - start) / T::from(3).unwrap(),
            end - (end - start) / T::from(3).unwrap(),
        );
        let (r1, r2) = (f(mid1), f(mid2));

        match r1.partial_cmp(&r2) {
            Some(Ordering::Less) => {
                return ternary_search_min_rec(f, start, mid2);
            }
            Some(Ordering::Greater) => {
                return ternary_search_min_rec(f, mid1, end);
            }
            Some(Ordering::Equal) => {
                return ternary_search_min_rec(f, mid1, mid2);
            }
            _ => return f(start),
        }
    }
    f(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_value() {
        let expected = 4.0;
        // let f = |x: f32| -x * x - 2.0 * x + 3.0;

        let start = -10000000000.0;
        let end = 10000000000.0;
        // let absolute_precision = f32::EPSILON;
        // let absolute_precision = 0.0000001;

        let result = ternary_search_max_rec::<f32>(|x| -x * x - 2.0 * x + 3.0, start, end);

        assert!(is_close!(result, expected));
        // assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value() {
        let expected = 2.0;
        let f = |x: f32| x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        // let absolute_precision = f32::EPSILON;
        // let absolute_precision = 0.0000001;

        let result = ternary_search_min_rec(f, start, end);

        assert!(is_close!(result, expected));
        // assert_eq!(result, expected);
    }

    #[test]
    fn finds_max_value_2() {
        let expected = 7.25;
        // let f = |x: f32| -x.powi(2) + 3.0 * x + 5.0;

        let start: f64 = -10000000000.0;
        let end: f64 = 10000000000.0;
        // let absolute_precision = f32::EPSILON;
        // let absolute_precision = 0.000001;

        let result = ternary_search_max_rec::<f64>(|x| -x.powi(2) + 3.0 * x + 5.0, start, end);

        assert!(is_close!(result, expected));
        // assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value_2() {
        let expected = 2.75;
        // let f = |x: f32| x.powi(2) + 3.0 * x + 5.0;

        let start: f64 = -10000000000.0;
        let end: f64 = 10000000000.0;
        // let absolute_precision = f64::EPSILON;
        // let absolute_precision = 0.000001;

        let result = ternary_search_min_rec::<f64>(|x| x.powi(2) + 3.0 * x + 5.0, start, end);

        assert!(is_close!(result, expected));
        // assert_eq!(result, expected);
    }
}
