// Given a string s containing just the characters
// '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
pub struct Solution;

fn main() {
    println!("Solution to Q20");
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // our technique will be to go through the string
        // character by character. we will push opening entries to the static
        // and pop entries when we get a closing entry. if the closing does
        // not match the opening, then return false. if at the end, the stack
        // has entries return false.
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == '{' || c == '[' || c == '(' {
                stack.push(c);
            }
            match c {
                ']' => match stack.pop() {
                    Some(popped_c) => {
                        if popped_c != '[' {
                            return false;
                        };
                    }
                    None => return false,
                },
                '}' => match stack.pop() {
                    Some(popped_c) => {
                        if popped_c != '{' {
                            return false;
                        };
                    }
                    None => return false,
                },
                ')' => match stack.pop() {
                    Some(popped_c) => {
                        if popped_c != '(' {
                            return false;
                        };
                    }
                    None => return false,
                },
                _ => {}
            }
        }
        return stack.is_empty();
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let output = solution::is_valid("()".to_string());
        assert!(output);
    }
    #[test]
    fn ex2() {
        let output = solution::is_valid("[]{}".to_string());
        assert!(output);
    }
    #[test]
    fn ex3() {
        let output = solution::is_valid("([{}])".to_string());
        assert!(output);
    }
    #[test]
    fn ex4() {
        let output = solution::is_valid("(]".to_string());
        assert!(!output);
    }
}
