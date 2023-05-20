// Given a binary search tree (BST), find the lowest common ancestor (LCA)
// node of two given nodes in the BST.
//
// According to the definition of LCA on Wikipedia: “The lowest common
// ancestor is defined between two nodes p and q as the lowest node in T
// that has both p and q as descendants (where we allow a node to be a
// descendant of itself).”


use std::rc::Rc;
use std::cell::RefCell;
use leetcode::TreeNode;
pub struct Solution;

fn main() {
    println!("Solution to Q");
}

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
       unimplemented!() 
    }
}


#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    #[test]
    fn ex1() {
        let root = vec![Some(6),Some(2),Some(8),Some(0),Some(4),Some(7),Some(9),None,None ,Some(3),Some(5)]
    }
}
