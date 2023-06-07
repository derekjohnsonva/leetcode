use std::collections::HashMap;

fn main() {
    println!("Solution to Q409");
}

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // My thinking is that you are only able to use even number letters
        // except for one time (the center of the palindrome).
        let mut letters_count: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|v| {
            letters_count
                .entry(v)
                .and_modify(|cur_count| *cur_count += 1)
                .or_insert(1);
        });
        let mut has_odd_flag = false;
        let mut longest = 0;
        for (_, val) in letters_count.into_iter() {
            longest += val - (val % 2);
            if val % 2 == 1 {
                has_odd_flag = true;
            }
        }
        if has_odd_flag {
            longest += 1;
        }
        longest
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let s = String::from("abccccdd");
        assert_eq!(Solution::longest_palindrome(s), 7);
    }
    #[test]
    fn ex2() {
        let s = String::from("a");
        assert_eq!(Solution::longest_palindrome(s), 1);
    }
    #[test]
    fn ex3() {
        let s = String::from("aA");
        assert_eq!(Solution::longest_palindrome(s), 1);
    }
    #[test]
    fn ex4() {
        let s = String::from("aAaB");
        assert_eq!(Solution::longest_palindrome(s), 3);
    }
}

// Example 1:
//
// Input: s = "abccccdd"
// Output: 7
// Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
//
// Example 2:
//
// Input: s = "a"
// Output: 1
// Explanation: The longest palindrome that can be built is "a", whose length is 1.
