use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indexes = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let remaining = target - num;
        match indexes.get(&remaining) {
            None => {
                indexes.insert(num, index);
                continue;
            }
            Some(start_index) => {
                return vec![*start_index as i32, index as i32];
            }
        }
    }
    vec![]
}

fn main() {
    let results = two_sum(vec![0, 1, 2, 4, 5], 6);
    println!("Numbers are {:?}", results);
}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
