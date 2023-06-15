use std::collections::HashSet;

fn main() {
    println!("Solution to Q217");
}

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // this is a pretty straightforward hash set question.
        // All we have to do is catalogue every entry we have seen
        let mut nums_seen: HashSet<i32> = HashSet::new();
        for num in nums {
            if !nums_seen.insert(num) {
                return true;
            } 
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let nums = vec![1,2,3,1];
        assert!(Solution::contains_duplicate(nums));
    }
    #[test]
    fn ex2() {
        let nums = vec![1,2,3,4];
        assert!(!Solution::contains_duplicate(nums));
    }
    #[test]
    fn ex3() {
        let nums = vec![1,1,1,3,3,4,3,2,4,2];
        assert!(Solution::contains_duplicate(nums));
    }

}
// Example 1:
//
// Input: nums = [1,2,3,1]
// Output: true
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: false
//
// Example 3:
//
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true
//
//  
