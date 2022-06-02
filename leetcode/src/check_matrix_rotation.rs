// 1886. Determine Whether Matrix Can Be Obtained By Rotation
//
// Given two n x n binary matrices mat and target,
// return true if it is possible to make mat equal to target by rotating mat in 90-degree increments, or false otherwise.
//
// Example 1:
// Input: mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
// Output: true
// Explanation: We can rotate mat 90 degrees clockwise to make mat equal target.
//
// Example 2:
// Input: mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
// Output: false
// Explanation: It is impossible to make mat equal to target by rotating mat.
//
// Example 3:
// Input: mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
// Output: true
// Explanation: We can rotate mat 90 degrees clockwise two times to make mat equal target.
//
// Constraints:
// n == mat.length == target.length
// n == mat[i].length == target[i].length
// 1 <= n <= 10
// mat[i][j] and target[i][j] are either 0 or 1.

struct Solution;

use std::mem;

fn transpose(m: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let size = m.len()-1;

    let directions = [(x, y), (y, size-x), (size-x, size-y), (size-y, x), (x, y)];
    directions.iter().fold(0, |acc, x| mem::replace(&mut m[x.0][x.1], acc));
}

fn rotate(mut mat :&mut Vec<Vec<i32>>) {
    let mlen = mat.len();
    for x in 0..mlen/2 {
        for y in x..(mlen-1)-x {
            transpose(&mut mat, x, y);
        }
    }
}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat = mat;
        (0..4).take_while(|_| {rotate(&mut mat); mat != target}).for_each(|_| ());
        mat == target
    }
}

#[test]
fn test_find_rotation() {
    assert_eq!(Solution::find_rotation(vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]], vec![vec![1,1,1],vec![0,1,0],vec![0,0,0]]), true);
}