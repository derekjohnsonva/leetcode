use std::collections::{HashMap, HashSet};

fn main() {
    println!("Solution to Q");
}

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // add all of the nums to a HashMap. The number should map to a vec containing the indeces
        // in which the number appears.
        let mut map = HashMap::new();
        // sorting helps us to not add the same set twice
        nums.sort_unstable();
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i);
        }
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut seen_a_vals = HashSet::new();
        for (i, num_a) in nums.iter().enumerate().take(nums.len() - 2) {
            // either add the num_a to seen_a_vals or skip over it
            if !seen_a_vals.insert(num_a) {
                continue;
            }
            let mut seen_b_vals = HashSet::new();
            for (b_index, num_b) in nums.iter().enumerate().skip(i + 1).take(nums.len() - 1) {
                if !seen_b_vals.insert(num_b) {
                    continue;
                }
                let num_c = -(num_a + num_b);
                // check to see if num_c is present in the map
                if let Some(c_indexes) = map.get(&num_c) {
                    // check to see if there is an index greater then b_index
                    for &_c_index in c_indexes.iter().filter(|&&c_index| c_index > b_index) {
                        results.push(vec![*num_a, *num_b, num_c]);
                        break;
                    }
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(output, Solution::three_sum(nums));
    }
    #[test]
    fn ex2() {
        let nums = vec![0, 1, 1];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(output, Solution::three_sum(nums));
    }
    #[test]
    fn ex3() {
        let nums = vec![0, 0, 0];
        let output = vec![vec![0, 0, 0]];
        assert_eq!(output, Solution::three_sum(nums));
    }
    #[test]
    fn ex4() {
        let nums = vec![0, 0, 0, 0];
        let output = vec![vec![0, 0, 0]];
        assert_eq!(output, Solution::three_sum(nums));
    }
}

// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
//
// Example 2:
//
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
//
// Example 3:
//
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
