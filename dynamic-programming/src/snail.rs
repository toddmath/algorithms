// use itertools::Itertools;

/// ## Spiral Sorting
///
/// Given an n x m array, return the array elements arranged from outermost
/// elements to the middle element, traveling INWARD FROM TOP-LEFT, CLOCKWISE.
pub fn snail<T: Copy>(matrix: &[Vec<T>]) -> Vec<T> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let (n_cols, n_rows) = (matrix[0].len(), matrix.len());
    let (mut max_col, mut max_row) = (n_cols - 1, n_rows - 1);
    let (mut min_col, mut min_row) = (0, 0);
    let (mut row, mut col) = (0, 0);
    let mut dir = Dir::Right;
    let mut result = Vec::with_capacity(n_rows * n_cols);

    while result.len() < n_rows * n_cols {
        result.push(matrix[row][col]);
        dir.snail_move(
            &mut col,
            &mut row,
            (&mut min_col, &mut max_col),
            (&mut min_row, &mut max_row),
        );
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

impl Dir {
    pub fn snail_move(
        &mut self,
        col: &mut usize,
        row: &mut usize,
        (min_col, max_col): (&mut usize, &mut usize),
        (min_row, max_row): (&mut usize, &mut usize),
    ) {
        match self {
            Self::Right => {
                *col = if *col < *max_col {
                    *col + 1
                } else {
                    *self = Self::Down;
                    *min_row += 1;
                    *row = *min_row;
                    *col
                };
            }
            Self::Down => {
                *row = if *row < *max_row {
                    *row + 1
                } else {
                    *self = Self::Left;
                    *max_col -= 1;
                    *col = *max_col;
                    *row
                };
            }
            Self::Left => {
                *col = if *col > usize::MIN && *col > *min_col {
                    *col - 1
                } else {
                    *self = Self::Up;
                    *max_row -= 1;
                    *row = *max_row;
                    *col
                };
            }
            Self::Up => {
                *row = if *row > usize::MIN && *row > *min_row {
                    *row - 1
                } else {
                    *self = Self::Right;
                    *min_col += 1;
                    *col = *min_col;
                    *row
                };
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let empty: &[Vec<i32>] = &[vec![]];
        assert_eq!(snail(empty), vec![]);
    }

    #[test]
    fn test_int() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(snail(square), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_char() {
        let square = &[vec!['S', 'O', 'M'], vec!['E', 'T', 'H'], vec![
            'I', 'N', 'G',
        ]];
        assert_eq!(snail(square), vec![
            'S', 'O', 'M', 'H', 'G', 'N', 'I', 'E', 'T'
        ]);
    }

    #[test]
    fn test_rect() {
        let square = &[vec!['H', 'E', 'L', 'L'], vec!['O', ' ', 'W', 'O'], vec![
            'R', 'L', 'D', ' ',
        ]];
        assert_eq!(snail(square), vec![
            'H', 'E', 'L', 'L', 'O', ' ', 'D', 'L', 'R', 'O', ' ', 'W'
        ]);
    }
}
