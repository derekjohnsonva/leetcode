use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Solution to Q543");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecResult {
    left_len: i32,
    right_len: i32,
    max: i32,
}

pub struct Solution;
impl Solution {
    fn diameter_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> RecResult {
        if root.is_none() {
            panic!("This method should never be called on a None value");
        }
        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();
        let mut left_len = 0;
        let mut right_len = 0;
        let mut global_max = 0;
        if left.is_some() {
            let left_result = Self::diameter_recursive(left);
            left_len = i32::max(left_result.left_len, left_result.right_len) + 1;
            global_max = i32::max(global_max, left_result.max);
        }
        if right.is_some() {
            let right_result = Self::diameter_recursive(right);
            right_len = i32::max(right_result.left_len, right_result.right_len) + 1;
            global_max = i32::max(global_max, right_result.max);
        }
        global_max = i32::max(global_max, left_len + right_len);
        RecResult {
            left_len,
            right_len,
            max: global_max,
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // We can solve this problem recursivly.
        let res = Solution::diameter_recursive(root);
        res.max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
        let root = Some(Rc::new(RefCell::new(TreeNode::new_from_vec(root))));
        assert_eq!(3, Solution::diameter_of_binary_tree(root));
    }
    #[test]
    fn ex2() {
        let root = vec![Some(1), Some(2)];
        let root = Some(Rc::new(RefCell::new(TreeNode::new_from_vec(root))));
        assert_eq!(1, Solution::diameter_of_binary_tree(root));
    }
}

// Input: root = [1,2,3,4,5]
// Output: 3
// Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
//
// Example 2:
//
// Input: root = [1,2]
// Output: 1
