// 51. N-Queens
// https://leetcode.com/problems/n-queens/
//
// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
//
// Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
//
// Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
//
// Example 1:
// Input: n = 4
// Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
//
// Example 2:
// Input: n = 1
// Output: [["Q"]]
//
// Constraints:
// 1 <= n <= 9

use std::fmt::{Display, Formatter};

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = Board::new(n as usize);
        board.multi_solve();
        board.solution
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Status {
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Board {
    size: usize,
    cells: Vec<Vec<Status>>,
    solution: Vec<Vec<String>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        self.cells.iter().for_each(|row| {
            write!(f, "{:?}\n", row);
            ()
        });
        write!(f, "")
    }
}

impl Board {
    fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![Status::Empty; size]; size],
            solution: vec![],
        }
    }

    fn is_safe(&self, x: usize, y: usize) -> bool {
        self.cells.iter().all(|row| row[y] == Status::Empty)
            && self.cells[x].iter().all(|&value| value == Status::Empty)
            && (0..x).into_iter().rev().all(|i| {
                (y < (x - i) || self.cells[i][y - (x - i)] == Status::Empty)
                    && (y + (x - i) >= self.size || self.cells[i][y + (x - i)] == Status::Empty)
            })
    }

    fn solve_util(&mut self, row: usize) -> bool {
        if row == self.size {
            return true;
        }
        for col in 0..self.size {
            if !self.is_safe(row, col) {
                continue;
            }
            self.cells[row][col] = Status::Occupied;
            if self.solve_util(row + 1) {
                return true;
            }
            self.cells[row][col] = Status::Empty;
        }
        false
    }

    fn solve(&mut self) {
        if self.solve_util(0) {
            println!("Solved board is {}", self);
        } else {
            println!("Un-Solved board is {}", self);
        }
    }

    fn to_output(&self) -> Vec<String> {
        let mut board: Vec<String> = vec![];
        self.cells.iter().for_each(|row| {
            let mut a: String = String::new();
            for i in row {
                a.push(match i {
                    Status::Empty => '.',
                    Status::Occupied => 'Q',
                })
            }
            board.push(a);
        });
        return board;
    }

    fn multi_solve_util(&mut self, row: usize) {
        if row == self.size {
            self.solution.push(self.to_output());
            return;
        }
        for col in 0..self.size {
            if !self.is_safe(row, col) {
                continue;
            }
            self.cells[row][col] = Status::Occupied;
            self.multi_solve_util(row + 1);
            self.cells[row][col] = Status::Empty;
        }
    }

    fn multi_solve(&mut self) -> bool {
        self.multi_solve_util(0);
        return self.solution.len() > 0;
    }
}

#[test]
fn test_multi_solve_board() {
    for i in 9..10 {
        let mut b = Board::new(i);
        b.multi_solve();
        println!("Solution is {:?}", b.solution);
    }
}

#[test]
fn test_solve_board() {
    for i in 1..10 {
        println!("Solving for {}", i);
        let mut b = Board::new(i);
        b.solve();
    }
}

#[test]
fn test_iter() {
    (0..=5)
        .into_iter()
        .inspect(|i| println!("iter {}", i))
        .all(|_| true);
    (0..=5)
        .into_iter()
        .rev()
        .inspect(|i| println!("iter {}", i))
        .all(|_| true);
}

#[test]
fn test_create_board() {
    let b = Board::new(1);
    assert_eq!(b.size, 1);
    assert_eq!(b.cells, vec![vec![Status::Empty]]);
    let b = Board::new(2);
    assert_eq!(b.size, 2);
    assert_eq!(b.cells, vec![vec![Status::Empty; 2]; 2]);
}

#[test]
fn test_solve_n_queens() {
    assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
    // assert_eq!(Solution::solve_n_queens(4), vec![vec![".Q..","...Q","Q...","..Q."], vec!["..Q.","Q...","...Q",".Q.."]]);
}
