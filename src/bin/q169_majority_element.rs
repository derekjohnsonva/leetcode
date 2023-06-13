use std::collections::HashMap;

fn main() {
    println!("Solution to Q169");
}

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // return the element that appears more than len / 2 times 
        let mut frequencies: HashMap<i32, i32> = HashMap::new(); 
        for num in nums.iter() {
            let cur_val = frequencies.entry(*num).and_modify(|cur_val| *cur_val += 1).or_insert(1);
            if *cur_val > (nums.len() as i32 / 2) {
                return *num;
            }
        }
        1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let nums = vec![3,2,3];
        assert_eq!(3, Solution::majority_element(nums));
    }
    #[test]
    fn ex2() {
        let nums = vec![2,2,1,1,1,2,2];
        assert_eq!(2, Solution::majority_element(nums));
    }
}
// Example 1:
//
// Input: nums = [3,2,3]
// Output: 3
//
// Example 2:
//
// Input: nums = [2,2,1,1,1,2,2]
// Output: 2
//
 
