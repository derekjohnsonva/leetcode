// Given a binary search tree (BST), find the lowest common ancestor (LCA)
// node of two given nodes in the BST.
//
// According to the definition of LCA on Wikipedia: “The lowest common
// ancestor is defined between two nodes p and q as the lowest node in T
// that has both p and q as descendants (where we allow a node to be a
// descendant of itself).”


use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use leetcode::TreeNode;
pub struct Solution;

fn main() {
    println!("Solution to Q");
}

impl Solution {
    fn dfs (root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Vec<i32>> {
        let queue = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if node.eq(&p) {
                
            } 
        }
        Some(Vec::new())
    }
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

    }
}


#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn ex1() {
        let root = vec![Some(6),Some(2),Some(8),Some(0),Some(4),Some(7),Some(9),None,None ,Some(3),Some(5)];
        let root = TreeNode::new_from_vec(root);
        let root = Some(Rc::new(RefCell::new(root)));
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let lca = Solution::lowest_common_ancestor(root, p, q);
        // Check that the val of lca is 6
        assert_eq!(lca.unwrap().borrow().val, 6);
    }
}
