use std::cell::RefCell;
use std::rc::Rc;
use leetcode::TreeNode;

fn main() {
    println!("Solution to Q");
}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(2000);
        Solution::helper(root, &mut result, 0);
        result
    }
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(node) = root {
            if result.len() == level {
                result.push(vec![]);
            }
            result[level].push(node.borrow().val);
            Solution::helper(node.borrow().left.clone(), result, level + 1);
            Solution::helper(node.borrow().right.clone(), result, level + 1);
        }
    }
}
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let root = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = Some(Rc::new(RefCell::new(TreeNode::new_from_vec(root))));
        let output = vec![vec![3],vec![9,20],vec![15,7]];
        assert_eq!(output, Solution::level_order(root));
    }
    #[test]
    fn ex2() {
        let root = vec![Some(3)];
        let root = Some(Rc::new(RefCell::new(TreeNode::new_from_vec(root))));
        let output = vec![vec![3]];
        assert_eq!(output, Solution::level_order(root));
    }
    #[test]
    fn ex3() {
        let root = None;
        let output: Vec<Vec<i32>> = Vec::new();
        assert_eq!(output, Solution::level_order(root));
    }
}

// Input: root = [3,9,20,null,null,15,7]
// Output: [[3],[9,20],[15,7]]
//
// Example 2:
//
// Input: root = [1]
// Output: [[1]]
//
// Example 3:
//
// Input: root = []
// Output: []
