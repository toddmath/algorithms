//! Sudoku solver using Backtracking.
//! [`GeeksForGeeks`]
//!
//! [`GeeksForGeeks`]: https://www.geeksforgeeks.org/sudoku-backtracking-7/

/// Sudoku board
#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    /// [Sudoku] constructor
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board }
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn check(&self, (y, x): (usize, usize), value: u8) -> bool {
        if (0..9).any(|i| self.board[i][x] == value || self.board[y][i] == value) {
            return false;
        }

        let (sec_row, sec_col) = (y / 3, x / 3);

        !((sec_row * 3)..(sec_row * 3 + 3))
            .flat_map(|i| ((sec_col * 3)..(sec_col * 3 + 3)).map(move |j| (i, j)))
            .any(|(i, j)| self.board[i][j] == value)
    }

    /// Solves the sudoku puzzle
    pub fn solve(&mut self) -> bool {
        match self.find_empty_cell() {
            Some((y, x)) => {
                for val in 1..10 {
                    if self.check((y, x), val) {
                        self.board[y][x] = val;
                        if self.solve() {
                            return true;
                        }
                        self.board[y][x] = 0;
                    }
                }
            }
            None => {
                return true;
            }
        }

        false
    }

    /// Helper function to display the board
    pub fn print_board(&self) {
        let p_3x1 = |arr: Vec<u8>, last: bool| {
            let str = arr
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if last {
                println!("{str}");
            } else {
                print!("{str} | ");
            }
        };

        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("- - - - - - - - - - - - - -");
            }
            p_3x1(self.board[i][0..3].to_vec(), false);
            p_3x1(self.board[i][3..6].to_vec(), false);
            p_3x1(self.board[i][6..9].to_vec(), true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_correct() {
        let board: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let board_result = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(is_solved);
        assert_eq!(sudoku.board, board_result);
    }

    #[test]
    fn test_sudoku_incorrect() {
        let board: [[u8; 9]; 9] = [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(!is_solved);
    }
}
