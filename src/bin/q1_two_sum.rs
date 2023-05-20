use std::collections::HashMap;

// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
//
// You can return the answer in any order.
fn main() {
    println!("Solution to Q1");
}
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // turn nums into a hashmap in the form key=num, val=index
        let mut nums_map: HashMap<&i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            nums_map.insert(num, index);
        }
        for (index, num) in nums.iter().enumerate() {
            let other_val = target - num;
            if let Some(other_index) = nums_map.get(&other_val) {
                if index != *other_index {
                    return vec![index as i32, *other_index as i32];
                }
            }
        }
        panic!("Could not find a solution")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![0, 1];
        assert_eq!(output, Solution::two_sum(nums, target));
    }
}
