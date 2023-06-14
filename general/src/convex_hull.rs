use std::cmp::Ordering;

use itertools::Itertools;
use num_traits::Float;

pub type Point<F> = (F, F);

fn sort_by_min_angle<F: Float>(points: &[Point<F>], &(min_x, min_y): &Point<F>) -> Vec<Point<F>> {
    points
        .iter()
        .map(|&(x, y)| {
            (
                (y - min_y).atan2(x - min_x),
                (y - min_y).hypot(x - min_x),
                (x, y),
            )
        })
        .sorted_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(_, _, x)| x)
        .collect_vec()
}

/// calculates the z coordinate of the vector product of vectors ab and ac
#[inline]
fn z_coord_vector_product<F: Float>(
    (ax, ay): Point<F>,
    (bx, by): Point<F>,
    (cx, cy): Point<F>,
) -> F {
    (bx - ax) * (cy - ay) - (cx - ax) * (by - ay)
}

/// If three points are aligned and are part of the convex hull then the three
/// are kept.  If one doesn't want to keep those points, it is easy to iterate
/// the answer and remove them  The first point is the one with the lowest
/// y-coordinate and the lowest x-coordinate.  Points are then given
/// counter-clockwise, and the closest one is given first if needed.
pub fn convex_hull_graham<F: Float>(points: &[Point<F>]) -> Vec<Point<F>> {
    if points.is_empty() {
        return vec![];
    }
    let mut stack = Vec::with_capacity(points.len());
    let min = points
        .iter()
        .min_by(|(ax, ay), (bx, by)| match ay.partial_cmp(by) {
            None => ax.partial_cmp(bx).unwrap_or(Ordering::Equal),
            Some(ord) => ord,
        })
        .unwrap_or_else(|| panic!("cannot find minimum value."));

    let points = sort_by_min_angle(points, min);
    if points.len() <= 3 {
        return points;
    }

    for point in points {
        while stack.len() > 1
            && z_coord_vector_product(stack[stack.len() - 2], stack[stack.len() - 1], point)
                < F::zero()
        {
            stack.pop();
        }
        stack.push(point);
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(convex_hull_graham::<f32>(&[]), vec![]);
    }

    #[test]
    fn not_enough_points() {
        let list = vec![(0f64, 0f64)];
        assert_eq!(convex_hull_graham(&list), list);
    }

    #[test]
    fn not_enough_points1() {
        let list = vec![(2f64, 2f64), (1f64, 1f64), (0f64, 0f64)];
        let ans = vec![(0f64, 0f64), (1f64, 1f64), (2f64, 2f64)];
        assert_eq!(convex_hull_graham(&list), ans);
    }

    #[test]
    fn not_enough_points2() {
        let list = vec![(2f64, 2f64), (1f64, 2f64), (0f64, 0f64)];
        let ans = vec![(0f64, 0f64), (2f64, 2f64), (1f64, 2f64)];
        assert_eq!(convex_hull_graham(&list), ans);
    }

    #[test]
    // from https://codegolf.stackexchange.com/questions/11035/find-the-convex-hull-of-a-set-of-2d-points
    fn lots_of_points() {
        let list = vec![
            (4.4, 14.),
            (6.7, 15.25),
            (6.9, 12.8),
            (2.1, 11.1),
            (9.5, 14.9),
            (13.2, 11.9),
            (10.3, 12.3),
            (6.8, 9.5),
            (3.3, 7.7),
            (0.6, 5.1),
            (5.3, 2.4),
            (8.45, 4.7),
            (11.5, 9.6),
            (13.8, 7.3),
            (12.9, 3.1),
            (11., 1.1),
        ];
        let ans = vec![
            (11., 1.1),
            (12.9, 3.1),
            (13.8, 7.3),
            (13.2, 11.9),
            (9.5, 14.9),
            (6.7, 15.25),
            (4.4, 14.),
            (2.1, 11.1),
            (0.6, 5.1),
            (5.3, 2.4),
        ];

        assert_eq!(convex_hull_graham(&list), ans);
    }

    #[test]
    // from https://codegolf.stackexchange.com/questions/11035/find-the-convex-hull-of-a-set-of-2d-points
    fn lots_of_points2() {
        let list = vec![
            (1., 0.),
            (1., 1.),
            (1., -1.),
            (0.68957, 0.283647),
            (0.909487, 0.644276),
            (0.0361877, 0.803816),
            (0.583004, 0.91555),
            (-0.748169, 0.210483),
            (-0.553528, -0.967036),
            (0.316709, -0.153861),
            (-0.79267, 0.585945),
            (-0.700164, -0.750994),
            (0.452273, -0.604434),
            (-0.79134, -0.249902),
            (-0.594918, -0.397574),
            (-0.547371, -0.434041),
            (0.958132, -0.499614),
            (0.039941, 0.0990732),
            (-0.891471, -0.464943),
            (0.513187, -0.457062),
            (-0.930053, 0.60341),
            (0.656995, 0.854205),
        ];
        let ans = vec![
            (1., -1.),
            (1., 0.),
            (1., 1.),
            (0.583004, 0.91555),
            (0.0361877, 0.803816),
            (-0.930053, 0.60341),
            (-0.891471, -0.464943),
            (-0.700164, -0.750994),
            (-0.553528, -0.967036),
        ];

        assert_eq!(convex_hull_graham(&list), ans);
    }
}
