/*
37. Sudoku Solver
https://leetcode.com/problems/sudoku-solver/

Write a program to solve a Sudoku puzzle by filling the empty cells.

A sudoku solution must satisfy all of the following rules:

Each of the digits 1-9 must occur exactly once in each row.
Each of the digits 1-9 must occur exactly once in each column.
Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
The '.' character indicates empty cells.

Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
Constraints:

board.length == 9
board[i].length == 9
board[i][j] is a digit or '.'.
It is guaranteed that the input board has only one solution.
*/

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Sudoku::from(board).solve();
    }
}

const NUMBERS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug, PartialEq)]
struct Sudoku<'a>(&'a mut Vec<Vec<char>>);

impl<'a> Sudoku<'a> {
    pub fn from(input: &'a mut Vec<Vec<char>>) -> Self {
        Self(input)
    }

    fn is_safe(&self, x: usize, y: usize, value: char) -> bool {
        // Find x and y of contained square
        let _x= (x / 3) * 3;
        let _y= (y / 3) * 3;

        !self.0[x].iter().any(|&cell| cell == value) &&
            !self.0.iter().any(|row| row[y] == value) &&
            !self.0[_x.._x+3].iter().any(|row| row[_y.._y+3].iter().any(|&cell| cell == value))
    }

    fn _solve(&mut self, x: usize, y: usize) -> bool {
        if x > 8 { return true; }

        let mut next_x = x;
        let mut next_y = y + 1;
        if next_y > 8 {
            next_x += 1;
            next_y = 0;
        }
        if self.0[x][y] != '.' {
            return self._solve(next_x, next_y)
        }

        for test_value in NUMBERS {
            if !self.is_safe(x, y, test_value) { continue; }
            self.0[x][y] = test_value;
            if self._solve(next_x, next_y) {
                return true;
            }
        }
        self.0[x][y] = '.';

        false
    }


    pub fn solve(&mut self) -> bool {
        self._solve(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_sample_data () -> (Vec<Vec<char>>, Vec<Vec<char>>) {
        let problem = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let solution = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
        ];
        return (problem.clone(), solution.clone())
    }

    #[test]
    fn test_sudoku() {
        let (mut problem, solution) = get_sample_data();
        let mut problem = Sudoku::from(&mut problem);

        assert_eq!(problem.is_safe(1, 1, '3'), false);
        assert_eq!(problem.is_safe(2, 1, '3'), false);
        assert_eq!(problem.is_safe(3, 2, '3'), false);
        assert_eq!(problem.is_safe(8, 8, '3'), false);
        assert_eq!(problem.is_safe(0, 2, '4'), true);
        assert_eq!(problem.is_safe(3, 1, '5'), true);
        assert_eq!(problem.is_safe(8, 6, '1'), true);

        assert!(problem.solve());
        assert_eq!(problem.0, &solution);
    }

    #[test]
    fn test_sudoku_using_solution() {
        let (mut problem, solution) = get_sample_data();
        Solution::solve_sudoku(&mut problem);
        assert_eq!(problem, solution);
    }
}
