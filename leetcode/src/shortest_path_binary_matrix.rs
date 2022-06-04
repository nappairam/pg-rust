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

struct Solution;

use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let grid_length = grid.len();
        // Handle corner case
        if grid[0][0] == 1 || grid[grid_length - 1][grid_length - 1] == 1 {
            return -1;
        }

        let mut queue = VecDeque::with_capacity(grid_length * grid_length);

        grid[0][0] = 1;
        queue.push_back((0, 0));

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let is_safe_index = |i| i >= 0 && i < grid_length as i32;
            let is_safe = |x: i32, y: i32| is_safe_index(x) && is_safe_index(y);

            for (next_x, next_y) in DIRECTIONS
                .iter()
                .map(|(_x, _y)| (x + _x, y + _y))
                .filter(|&(_x, _y)| is_safe(_x, _y))
            {
                if grid[next_x as usize][next_y as usize] != 0 {
                    continue;
                }
                grid[next_x as usize][next_y as usize] = grid[x as usize][y as usize] + 1;
                queue.push_back((next_x, next_y));
            }
        }

        if 0 != grid[grid_length - 1][grid_length - 1] {
            grid[grid_length - 1][grid_length - 1]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);
    }
    #[test]
    fn test_shortest_path_with_no_starting_point() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
    }
    #[test]
    fn test_shortest_path_no_path() {
        let grid = vec![vec![0, 1, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
    }
}
