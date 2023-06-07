use std::collections::HashMap;

fn main() {
    println!("Solution to Q383");
}
pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // make a hashmap from the values in the ransom note
        let mut magazine_vals: HashMap<char, i32> = HashMap::new();
        magazine.chars().for_each(|v| {
            magazine_vals
                .entry(v)
                .and_modify(|cur_count| *cur_count += 1)
                .or_insert(1);
            println!("{} maps to {}", v, magazine_vals.get(&v).unwrap());
        });
        for ch in ransom_note.chars() {
            match magazine_vals.get(&ch) {
                Some(val) => {
                    if *val <= 0 {
                        return false;
                    }
                    magazine_vals.insert(ch, *val - 1);
                },
                None => {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let note = String::from("a");
        let mag = String::from("b");
        assert!(!Solution::can_construct(note, mag));
    }
    #[test]
    fn ex2() {
        let note = String::from("aa");
        let mag = String::from("ab");
        assert!(!Solution::can_construct(note, mag));
    }
    #[test]
    fn ex3() {
        let note = String::from("aa");
        let mag = String::from("aab");
        assert!(Solution::can_construct(note, mag));
    }
}

// Example 1:
//
// Input: ransomNote = "a", magazine = "b"
// Output: false
//
// Example 2:
//
// Input: ransomNote = "aa", magazine = "ab"
// Output: false
//
// Example 3:
//
// Input: ransomNote = "aa", magazine = "aab"
// Output: true
