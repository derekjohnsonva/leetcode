fn main() {
    println!("Solution to Q");
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens.iter() {
            match token.as_str() {
                "+" => {
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();
                    stack.push(v1 + v2)
                },
                "-" => {
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();
                    stack.push(v1 - v2)
                },
 
                "*" => {
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();
                    stack.push(v1 * v2)
                },

                "/" => {
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();
                    stack.push(v1 / v2)
                },

                _ => stack.push(token.parse().unwrap())
            }
        }
        stack.pop().unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        assert_eq!(9, Solution::eval_rpn(tokens));
    }
    #[test]
    fn ex2() {
        let tokens = vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ];
        assert_eq!(6, Solution::eval_rpn(tokens));
    }
    #[test]
    fn ex3() {
        let tokens = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];
        assert_eq!(22, Solution::eval_rpn(tokens));
    }
}

// Input: tokens = ["2","1","+","3","*"]
// Output: 9
// Explanation: ((2 + 1) * 3) = 9
//
// Example 2:
//
// Input: tokens = ["4","13","5","/","+"]
// Output: 6
// Explanation: (4 + (13 / 5)) = 6
//
// Example 3:
//
// Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
// Output: 22
// Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
// = ((10 * (6 / (12 * -11))) + 17) + 5
// = ((10 * (6 / -132)) + 17) + 5
// = ((10 * 0) + 17) + 5
// = (0 + 17) + 5
// = 17 + 5
// = 22
