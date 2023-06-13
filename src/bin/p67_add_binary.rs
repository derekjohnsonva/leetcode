fn main() {
    println!("Solution to Q");
}

pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_iter = a.chars().rev();
        let mut b_iter = b.chars().rev();
        let mut output: String = String::new();
        let mut carry = false;
        let mut exit = false;
        while !exit {
            let a_val = a_iter.next();
            let b_val = b_iter.next();
            if a_val.is_none() && b_val.is_none() {
                exit = true;
            } else {
                // condition in which we have at least one value
                let a_bit = a_val.unwrap_or('0') == '1';
                let b_bit = b_val.unwrap_or('0') == '1';
                let sum = (a_bit ^ b_bit) ^ carry;                
                carry = a_bit & b_bit | (carry & (a_bit ^ b_bit)); 
                let sum_char = if sum {'1'} else {'0'};
                output.push(sum_char);
            }
        }
        if carry {
            output.push('1');
        }
        // reverse the string
        output.chars().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let a = String::from("11");
        let b = String::from("1");
        let out = String::from("100");
        assert_eq!(out, Solution::add_binary(a, b));
    }
    #[test]
    fn ex2() {
        let a = String::from("1010");
        let b = String::from("1011");
        let out = String::from("10101");
        assert_eq!(out, Solution::add_binary(a, b));
    }
}

// Example 1:
//
// Input: a = "11", b = "1"
// Output: "100"
//
// Example 2:
//
// Input: a = "1010", b = "1011"
// Output: "10101"
