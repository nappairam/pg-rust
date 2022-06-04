// 867. Transpose Matrix
// https://leetcode.com/problems/transpose-matrix/
//
// Given a 2D integer array matrix, return the transpose of matrix.
// The transpose of a matrix is the matrix flipped over its main diagonal,
// switching the matrix's row and column indices.
//
// Example 1:
// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[1,4,7],[2,5,8],[3,6,9]]
//
// Example 2:
// Input: matrix = [[1,2,3],[4,5,6]]
// Output: [[1,4],[2,5],[3,6]]
//
// Constraints:
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 1000
// 1 <= m * n <= 105
// -109 <= matrix[i][j] <= 109

struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let x_len = matrix.len();
        let y_len = matrix[0].len();
        let mut result = vec![vec![1; x_len]; y_len];
        for x_index in 0..x_len {
            for y_index in 0..y_len {
                result[y_index][x_index] = matrix[x_index][y_index];
            }
        }
        // println!("Result is {:?}", result);
        result
    }
}

#[test]
fn test_transpose() {
    assert_eq!(
        Solution::transpose(vec![vec![1, 0], vec![2, 3], vec![4, 5]]),
        vec![vec![1, 2, 4], vec![0, 3, 5]]
    );
}
