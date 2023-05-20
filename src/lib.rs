use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn new_from_vec(vals: Vec<Option<i32>>) -> Self {
        if vals.is_empty() {
            panic!("Can't pass an empty vector");
        }

        let mut root = TreeNode::new(vals[0].unwrap());
        let mut queue = vec![(&mut root, 0)];

        while !queue.is_empty() {
            let (node, idx) = queue.remove(0);

            let left_idx = idx * 2 + 1;
            let right_idx = idx * 2 + 2;

            if left_idx < vals.len() && vals[left_idx].is_some() {
                let left_node = Rc::new(RefCell::new(TreeNode::new(
                    vals[left_idx].unwrap(),
                )));
                node.borrow_mut().left = Some(left_node.clone());
                queue.push((left_node.as_ref().borrow_mut().deref_mut(), left_idx));
            }

            if right_idx < vals.len() && vals[right_idx].is_some() {
                let right_node = Rc::new(RefCell::new(TreeNode::new(
                    vals[left_idx].unwrap(),
                )));
                node.borrow_mut().left = Some(right_node.clone());
                queue.push((right_node.as_ref().borrow_mut().deref_mut(), left_idx));
            }
        }
        root
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;
    #[test]
    fn ex1() {
        let root = vec![Some(6), Some(2), Some(8)];
        let root = TreeNode::new_from_vec(root);
        assert_eq!(root.val, 6);
        assert_eq!(root.left.unwrap().borrow().val, 2);
        assert_eq!(root.right.unwrap().borrow().val, 8);
    }
}
