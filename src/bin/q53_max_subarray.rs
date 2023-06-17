fn main() {
    println!("Solution to Q");
}

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur_max = i32::MIN;
        // basic idea is that we will use a sliding window with 2 rules
        // 1) Expand the end of the window when the cur_sum is greater than 0
        // 2) shrink the start of the window when the cur_sum is less than 0
        let mut cur_sum = 0;
        let (mut start, mut end) = (0, 0);
        while end < nums.len() {
            println!("{start}, {end}");
            cur_sum += nums[end];
            println!("Cur sum = {cur_sum}");
            cur_max = i32::max(cur_max, cur_sum);
            end += 1;
            while cur_sum <= 0 && start < end {
                cur_sum -= nums[start];
                start += 1;
            }
        }
        cur_max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(6, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex2() {
        let nums = vec![1];
        assert_eq!(1, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex3() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(23, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex4() {
        let nums = vec![-1];
        assert_eq!(-1, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex5() {
        let nums = vec![-2, -5, -1];
        assert_eq!(-1, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex6() {
        let nums = vec![-2, -5, 6, -4, -1, 1];
        assert_eq!(6, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex7() {
        let nums = vec![-2, 3, 1, 3];
        assert_eq!(7, Solution::max_sub_array(nums));
    }
    #[test]
    fn ex8() {
        let nums = vec![-1, 2, 2, -3];
        assert_eq!(4, Solution::max_sub_array(nums));
    }
}

// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//
// Example 2:
//
// Input: nums = [1]
// Output: 1
// Explanation: The subarray [1] has the largest sum 1.
//
// Example 3:
//
// Input: nums = [5,4,-1,7,8]
// Output: 23
// Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
