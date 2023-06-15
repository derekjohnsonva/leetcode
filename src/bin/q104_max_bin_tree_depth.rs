use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Solution to Q104");
}

pub struct Solution;

impl Solution {
    fn max_depth_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + i32::max(
                    Self::max_depth_rec(node.borrow().left.clone()),
                    Self::max_depth_rec(node.borrow().right.clone()),
                )
            }
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth_rec(root)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let root = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = TreeNode::new_from_vec(root);
        assert_eq!(3, Solution::max_depth(Some(Rc::new(RefCell::new(root)))));
    }
    #[test]
    fn ex2() {
        let root = vec![Some(3), None, Some(2)];
        let root = TreeNode::new_from_vec(root);
        assert_eq!(2, Solution::max_depth(Some(Rc::new(RefCell::new(root)))));
    }
}

// Input: root = [3,9,20,null,null,15,7]
// Output: 3
//
// Example 2:
//
// Input: root = [1,null,2]
// Output: 2
