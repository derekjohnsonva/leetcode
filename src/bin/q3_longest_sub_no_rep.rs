
fn main() {
    println!("Solution to Q");
}

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // We can keep track of two pointers, one at the start of the substring and one at the end.
        // We will also keep a HashSet that will keep track of the characters that have been seen.
        // Every time we expand the head, we add the character the the HashSet. We continue to
        // expand the head until we find a character that is already in the HashSet. At this point,
        // we will increment the tail pointer until we are either at the first appearance of the
        // character or the head pointer.
        let mut cur_max = 0;
        let mut char_positions = [0; 128];
        let mut tail = 0;
        for (head, head_char) in s.chars().enumerate() {
            tail = tail.max(char_positions[head_char as usize]);
            char_positions[head_char as usize] = head + 1;
            cur_max = cur_max.max((head - tail + 1) as i32);
        }
        cur_max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let input = "abcabcbb";
        let output = 3;
        assert_eq!(
            output,
            Solution::length_of_longest_substring(input.to_string())
        );
    }
    #[test]
    fn ex2() {
        let input = "bbbbb";
        let output = 1;
        assert_eq!(
            output,
            Solution::length_of_longest_substring(input.to_string())
        );
    }
    #[test]
    fn ex3() {
        let input = "pwwkew";
        let output = 3;
        assert_eq!(
            output,
            Solution::length_of_longest_substring(input.to_string())
        );
    }
    #[test]
    fn ex4() {
        let input = "abba";
        let output = 2;
        assert_eq!(
            output,
            Solution::length_of_longest_substring(input.to_string())
        );
    }
}

// xample 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
//
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
//
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
