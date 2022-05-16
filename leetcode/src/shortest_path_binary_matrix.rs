// 1091. Shortest Path in Binary Matrix
// https://leetcode.com/problems/shortest-path-in-binary-matrix/
//
// Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
//
// A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
//
// All the visited cells of the path are 0.
// All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
// The length of a clear path is the number of visited cells of this path.

// Constraints:
//
// n == grid.length
// n == grid[i].length
// 1 <= n <= 100
// grid[i][j] is 0 or 1

use std::collections::{HashSet, VecDeque};

struct Solution;

const OFFSETS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let grid_length = grid.len();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::with_capacity(grid_length);

        queue.push_back((0, 0, 1));
        visited.insert(1);

        // Handle corner case
        if grid[0][0] == 1 || grid[grid_length-1][grid_length-1] == 1 {
            return -1;
        }


        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 0);
    }
}