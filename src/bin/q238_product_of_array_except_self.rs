fn main() {
    println!("Solution to Q");
}
// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using the division operation.
pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // Keep track of the multiples on either side of a number
        // Do this in two arrays
        let mut proc_before = vec!(1; nums.len());
        let mut proc_after = vec!(1; nums.len());
        for i in 1..nums.len() {
            proc_before[i] = proc_before[i - 1] * nums[i - 1];
        }
        for i in (0..(nums.len()-1)).rev() {
            proc_after[i] = proc_after[i + 1] * nums[i + 1];
        } 
        let mut output = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            output.push(proc_before[i] * proc_after[i]);
        }
        output 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let nums = vec![1, 2, 3, 4];
        let output = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), output);
    }
    #[test]
    fn ex2() {
        let nums = vec![-1,1,0,-3,3];
        let output = vec![0,0,9,0,0];
        assert_eq!(Solution::product_except_self(nums), output);
    }
}
