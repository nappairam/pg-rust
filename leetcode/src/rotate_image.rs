// 48. Rotate Image
// https://leetcode.com/problems/rotate-image/
//
// You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
// You have to rotate the image in-place, which means you have to modify the input 2D matrix directly.
// DO NOT allocate another 2D matrix and do the rotation.
//
// Example 1:
// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[7,4,1],[8,5,2],[9,6,3]]
//
// Example 2:
// Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
// Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
//
// Constraints:
// n == matrix.length == matrix[i].length
// 1 <= n <= 20
// -1000 <= matrix[i][j] <= 1000

// 0, 0 <-> 0, 2 <-> 2, 2 <-> 2, 0
// 0, 1 <-> 1, 2 <-> 2, 1 <-> 1, 0
//
// 0, 0 <-> 0, 3 <-> 3, 3 <-> 3, 0
// 0, 1 <-> 1, 3 <-> 3, 2 <-> 2, 0
// 0, 2 <-> 2, 3 <-> 3, 1 <-> 1, 0
// 1, 1 <-> 1, 2 <-> 2, 2 <-> 2, 1

use std::mem;

struct Solution;

#[derive(PartialOrd, PartialEq)]
enum Direction {
    Clockwise,
    AntiClockwise,
}

fn transpose(m: &mut Vec<Vec<i32>>, x: usize, y: usize, dir: Direction) {
    let size = m.len()-1;

    // let temp = m[x][y];
    // println!("({},{}) ({},{}) ({}, {}) ({}, {})", x, y, size-y, x, size-x, size-y, y, size-x);
    // m[x][y] = m[size-y][x];
    // m[size-y][x] = m[size-x][size-y];
    // m[size-x][size-y] = m[y][size-x];
    // m[y][size-x] = temp;
    // let mut directions = [(size-y, x), (size-x, size-y), (y, size-x), (x, y)];

    let mut directions = [(x, y), (y, size-x), (size-x, size-y), (size-y, x), (x, y)];
    if dir == Direction::AntiClockwise {directions.reverse();}

    directions.iter().fold(0, |acc, x| mem::replace(&mut m[x.0][x.1], acc));
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mlen = matrix.len();
        for x in 0..mlen/2 {
            // println!("Looping x {} till {}", x, (mlen-1)-x);
            for y in x..(mlen-1)-x {
                transpose(matrix, x, y, Direction::Clockwise);
            }
        }
    }
}

#[test]
fn test_rotate_one() {
    let mut vect = vec![vec![1, 2, 3],vec![4, 5, 6],vec![7, 8, 9]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![7, 4, 1],vec![8, 5, 2],vec![9, 6, 3]]);
}

#[test]
fn test_rotate() {
    let mut vect = vec![vec![1]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![1]]);

    let mut vect = vec![vec![1, 2],vec![3, 4]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![3, 1],vec![4, 2]]);

    let mut vect = vec![vec![1, 2, 3],vec![4, 5, 6],vec![7, 8, 9]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![7, 4, 1],vec![8, 5, 2],vec![9, 6, 3]]);

    let mut vect = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]]);

    let mut vect = vec![vec![1,2,3,4,5],vec![6,7,8,9,10],vec![11,12,13,14,15],vec![16,17,18,19,20],vec![21,22,23,24,25]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![21,16,11,6,1],vec![22,17,12,7,2],vec![23,18,13,8,3],vec![24,19,14,9,4],vec![25,20,15,10,5]]);

    let mut vect = vec![vec![2,29,20,26,16,28],vec![12,27,9,25,13,21],vec![32,33,32,2,28,14],vec![13,14,32,27,22,26],vec![33,1,20,7,21,7],vec![4,24,1,6,32,34]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![4,33,13,32,12,2],vec![24,1,14,33,27,29],vec![1,20,32,32,9,20],vec![6,7,27,2,25,26],vec![32,21,22,28,13,16],vec![34,7,26,14,21,28]]);

    let mut vect = vec![vec![24,4,38,2,21,18,33,40],vec![24,37,25,62,37,15,35,36],vec![42,23,13,58,17,26,19,8],vec![32,48,9,58,36,18,40,61],vec![23,16,0,46,35,34,23,60],vec![9,49,60,47,1,32,20,45],vec![56,34,40,11,61,60,20,30],vec![45,30,25,18,49,3,16,10]];
    Solution::rotate(&mut vect);
    println!("transpose vector is {:?}", vect);
    assert_eq!(vect, vec![vec![45,56,9,23,32,42,24,24],vec![30,34,49,16,48,23,37,4],vec![25,40,60,0,9,13,25,38],vec![18,11,47,46,58,58,62,2],vec![49,61,1,35,36,17,37,21],vec![3,60,32,34,18,26,15,18],vec![16,20,20,23,40,19,35,33],vec![10,30,45,60,61,8,36,40]]);
}

#[test]
fn test_transpose() {
    let mut vect = vec![vec![1, 2, 3],vec![4, 5, 6],vec![7, 8, 9]];
    let expected = vect.clone();

    transpose(&mut vect, 0, 0, Direction::Clockwise);
    transpose(&mut vect, 0, 0, Direction::AntiClockwise);
    assert_eq!(vect, expected);

    transpose(&mut vect, 0, 0, Direction::Clockwise);
    transpose(&mut vect, 0, 1, Direction::Clockwise);
    transpose(&mut vect, 0, 0, Direction::AntiClockwise);
    transpose(&mut vect, 0, 1, Direction::AntiClockwise);
    assert_eq!(vect, expected);
}