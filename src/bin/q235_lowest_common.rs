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
        assert_ne!(p, q);
        assert!(root.is_some());
        let root_val = root.clone().unwrap();
        let root_val = root_val.borrow();
        let mut r = (root_val.val == p, root_val.val == q, None);
        let mut left_result = (false, false, None);
        let mut right_result = (false, false, None);

        if root_val.left.is_some() {
            left_result = Self::search(root_val.left.clone(), p, q);
        }

        if root_val.right.is_some() {
            right_result = Self::search(root_val.right.clone(), p, q);
        }
        r.0 = r.0 || left_result.0 || right_result.0;
        r.1 = r.1 || left_result.1 || right_result.1;
        if left_result.2.is_some() {
            r.2 = left_result.2;
        } else if right_result.2.is_some() {
            r.2 = right_result.2;
        } else {
            if r.0 && r.1 {
                r.2 = root;
            }
        }
        return r;
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let r = Self::search(root, p.unwrap().borrow().val, q.unwrap().borrow().val);
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
