// 63. Unique Paths II
// https://leetcode.com/problems/unique-paths-ii/
// You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m-1][n-1]). The robot can only move either down or right at any point in time.
//
// An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.
//
// Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
//
// The testcases are generated so that the answer will be less than or equal to 2 * 109.
//
//
//
// Example 1:
// Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
// Output: 2
// Explanation: There is one obstacle in the middle of the 3x3 grid above.
// There are two ways to reach the bottom-right corner:
// 1. Right -> Right -> Down -> Down
// 2. Down -> Down -> Right -> Right
// Example 2:
// Input: obstacleGrid = [[0,1],[0,0]]
// Output: 1
//
//
// Constraints:
//
// m == obstacleGrid.length
// n == obstacleGrid[i].length
// 1 <= m, n <= 100
// obstacleGrid[i][j] is 0 or 1.

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut path_grid = vec![vec![0; n]; m];
        path_grid[m - 1][n - 1] = 1;
        for x in (0..m).rev() {
            for y in (0..n).rev() {
                if obstacle_grid[x][y] == 1 {
                    continue
                }
                if x + 1 < m && obstacle_grid[x + 1][y] == 0 {
                    path_grid[x][y] += path_grid[x + 1][y];
                }

                if y + 1 < n && obstacle_grid[x][y + 1] == 0 {
                    path_grid[x][y] += path_grid[x][y + 1];
                }
            }
        }
        // println!("{:?}", path_grid);
        path_grid[0][0]
    }

    pub fn unique_paths_with_obstacles1(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut unique_paths = 0;
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return unique_paths;
        }


        fn find_unique_paths(grid: &Vec<Vec<i32>>, mut paths: &mut i32, x: usize, y: usize) -> bool {
            let m = grid.len();
            let n = grid[0].len();
            let is_safe = |x, y| x < m && y < n;
            println!("Find unique paths for {},{}", x, y);

            if x == m - 1 && y == n - 1 {
                println!("Solution found for {},{}", x, y);
                *paths += 1;
                panic!("panic");
                // return true;
            }

            if is_safe(x + 1, y) && grid[x + 1][y] == 0 {
                find_unique_paths(grid, &mut paths, x + 1, y);
            }

            if is_safe(x, y + 1) && grid[x][y + 1] == 0 {
                find_unique_paths(grid, &mut paths, x, y + 1);
            }

            return false;
        }
        find_unique_paths(&obstacle_grid, &mut unique_paths, 0, 0);
        unique_paths
    }
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_ii::Solution;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths_with_obstacles1(vec![vec![1]]), 0);
    }

    #[test]
    fn test_unique_paths_simple() {
        assert_eq!(Solution::unique_paths_with_obstacles(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
                   2
        );
    }

    #[test]
    fn test_unique_paths_big() {
        assert_eq!(Solution::unique_paths_with_obstacles(
            vec![
                vec![0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1],
                vec![0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            ]
        ),
                   13594824
        );
    }
}