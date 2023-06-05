// Given a binary search tree (BST), find the lowest common ancestor (LCA)
// node of two given nodes in the BST.
//
// According to the definition of LCA on Wikipedia: “The lowest common
// ancestor is defined between two nodes p and q as the lowest node in T
// that has both p and q as descendants (where we allow a node to be a
// descendant of itself).”

use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

fn main() {
    println!("Solution to Q");
}

impl Solution {
    fn search(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
        // Check to see if we have hit a None value
        if root.is_none() {
            return (false, false, None);
        }
        // Now check to see if the current node is one of the values p q
        let root_val = root.clone().unwrap();
        let root_val = root_val.borrow();
        let mut r = (root_val.val == p, root_val.val == q, None);
        // check the left node 
        let left_result = Self::search(root_val.left.clone(), p, q);
        // Check to see if the left search returned the lowest common ancestor 
        if left_result.2.is_some() {
            return left_result;
        }
        // Check the right node
        let right_result = Self::search(root_val.right.clone(), p, q);
        r.0 = r.0 || left_result.0 || right_result.0;
        r.1 = r.1 || left_result.1 || right_result.1;
        // Ceck to see if the right search returned the lowest common ancestor
        if right_result.2.is_some() {
            return right_result; 
        }
        // If the lowest common ancestor is not the left or the right,
        // we are either at a dead end or we are at the the lca
        // Check to see if both p and q have been seen at this node.
        if r.0 && r.1 {
            r.2 = root;
        }
        return r;
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        assert_ne!(p_val, q_val);
        let r = Self::search(root, p_val, q_val);
        r.2
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn ex1() {
        let root = vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ];
        let root = TreeNode::new_from_vec(root);
        let root = Some(Rc::new(RefCell::new(root)));
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let lca = Solution::lowest_common_ancestor(root, p, q);
        // Check that the val of lca is 6
        assert_eq!(lca.unwrap().borrow().val, 6);
    }
    #[test]
    fn ex2() {
        let root = vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ];
        let root = TreeNode::new_from_vec(root);
        let root = Some(Rc::new(RefCell::new(root)));
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let lca = Solution::lowest_common_ancestor(root, p, q);
        // Check that the val of lca is 6
        assert_eq!(lca.unwrap().borrow().val, 2);
    }
}
