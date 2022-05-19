// 329. Longest Increasing Path in a Matrix
// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
//
// Given an m x n integers matrix, return the length of the longest increasing path in matrix.
//
// From each cell, you can either move in four directions: left, right, up, or down.
// You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
//
// Example 1:
// Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
// Output: 4
// Explanation: The longest increasing path is [1, 2, 6, 9].
//
// Example 2:
// Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
// Output: 4
// Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
// Example 3:
//
// Input: matrix = [[1]]
// Output: 1
//
// Constraints:
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 200
// 0 <= matrix[i][j] <= 231 - 1

struct Solution;

use std::cmp;

const POSSIBLE_POSITIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    fn find_path(mat: &Vec<Vec<i32>>, paths: &mut Vec<Vec<i32>>, x: usize, y: usize) {
        let m = mat.len();
        let n = mat[0].len();
        if paths[x][y] != 0 {
            return;
        }

        let is_valid_index = |max, n| n >= 0 && n < max;
        let mut biggest = true;

        for position in POSSIBLE_POSITIONS.iter()
            .map(|&(_x, _y)| (x as i32 + _x , y as i32 + _y ))
            .filter(|&(_x, _y)| is_valid_index(m as i32, _x) && is_valid_index(n as i32, _y))
        {
            let _x = position.0 as usize;
            let _y = position.1 as usize;

            if mat[_x][_y] < mat[x][y] {
                biggest = false;
                Solution::find_path(mat, paths, _x, _y);
                paths[x][y] = cmp::max(paths[_x][_y]+1, paths[x][y])
            }
        }
        if biggest {
            paths[x][y] = 1;
        }
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut paths = vec![vec![0;n]; m];

        for x in 0..m {
            for y in 0..n {
                Solution::find_path(&matrix, &mut paths, x, y);
            }
        }
        // dbg!(paths);
        paths.iter().fold(0, |max, row| cmp::max(max, row.iter().fold(0, |max, &n| cmp::max(max, n))))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix_longest_inc_path::Solution;

    #[test]
    fn test_longest_inc_path() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![9,9,4], vec![6,6,8], vec![2,1,1]]), 4);
        assert_eq!(Solution::longest_increasing_path(vec![vec![3,4,5], vec![3,2,6], vec![2,2,1]]), 4);
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }
}
