// 304. Range Sum Query 2D - Immutable
// https://leetcode.com/problems/range-sum-query-2d-immutable/
//
// Given a 2D matrix matrix, handle multiple queries of the following type:
//
// Calculate the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
// Implement the NumMatrix class:
//
// NumMatrix(int[][] matrix) Initializes the object with the integer matrix matrix.
// int sumRegion(int row1, int col1, int row2, int col2) Returns the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
//
// Example 1:
// Input
// ["NumMatrix", "sumRegion", "sumRegion", "sumRegion"]
// [[[[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]], [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]]
// Output
// [null, 8, 11, 12]
//
// Explanation
// NumMatrix numMatrix = new NumMatrix([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
// numMatrix.sumRegion(2, 1, 4, 3); // return 8 (i.e sum of the red rectangle)
// numMatrix.sumRegion(1, 1, 2, 2); // return 11 (i.e sum of the green rectangle)
// numMatrix.sumRegion(1, 2, 2, 4); // return 12 (i.e sum of the blue rectangle)
//
//
// Constraints:
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 200
// -105 <= matrix[i][j] <= 105
// 0 <= row1 <= row2 < m
// 0 <= col1 <= col2 < n
// At most 104 calls will be made to sumRegion

#[derive(Debug, PartialOrd, PartialEq)]
struct NumMatrix {
    m: Vec<Vec<i32>>,
    c: Vec<Vec<i32>>,
}


/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead
  */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut computed = vec![vec![0; matrix[0].len()]; matrix.len()];
        for x in 0..matrix.len() {
            for y in 0..matrix[0].len() {
                computed[x][y] += matrix[x][y];
                if x != 0 {
                    computed[x][y] += computed[x-1][y];
                }
                if y != 0 {
                    computed[x][y] += computed[x][y-1];
                }
                if x != 0 && y!= 0 {
                    computed[x][y] -= computed[x-1][y-1];
                }
            }
        }
        Self {m: matrix, c: computed}
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        let mut sum = self.c[row2][col2];
        if row1 != 0 {
            sum -= self.c[row1-1][col2];
        }
        if col1 != 0 {
            sum -= self.c[row2][col1-1];
        }
        if row1 != 0 && col1 != 0 {
            sum += self.c[row1-1][col1-1];
        }
        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
#[test]
fn test_sum_region() {
    let matrix = vec![vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]];
    let nm = NumMatrix::new(matrix);
    println!("{:?}", nm);

    assert_eq!(nm.sum_region(1,1, 3, 3), 9);

}