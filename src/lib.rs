use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
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

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::with_capacity(vals.len());
        queue.push_back((Rc::clone(&root), 0));

        while let Some((node, idx)) = queue.pop_front() {
            let left_idx = idx * 2 + 1;
            let right_idx = idx * 2 + 2;

            if left_idx < vals.len() && vals[left_idx].is_some() {
                let left_node = Rc::new(RefCell::new(TreeNode::new(vals[left_idx].unwrap())));
                // set the left value of node to left_node
                node.as_ref().borrow_mut().deref_mut().left = Some(Rc::clone(&left_node));
                queue.push_back((Rc::clone(&left_node), left_idx));
            }

            if right_idx < vals.len() && vals[right_idx].is_some() {
                let right_node = Rc::new(RefCell::new(TreeNode::new(vals[right_idx].unwrap())));
                // set the right value of node to right_node
                node.as_ref().borrow_mut().deref_mut().right = Some(Rc::clone(&right_node));
                queue.push_back((Rc::clone(&right_node), right_idx));
            }
        }

        Rc::try_unwrap(root).unwrap().into_inner()
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
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
    #[test]
    fn ex2() {
        let root = vec![Some(6), None, Some(8)];
        let root = TreeNode::new_from_vec(root);
        assert_eq!(root.val, 6);
        assert!(root.left.is_none());
        assert_eq!(root.right.unwrap().borrow().val, 8);
    }
    #[test]
    fn ex3() {
        let root = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ];
        let root = TreeNode::new_from_vec(root);

        assert_eq!(root.val, 1);

        // Check all the other values
        assert_eq!(root.left.as_ref().unwrap().borrow().val, 2);
        assert_eq!(root.right.as_ref().unwrap().borrow().val, 3);
        assert_eq!(
            root.left
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );
        assert_eq!(
            root.left
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            5
        );
        assert_eq!(
            root.right
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            6
        );
        assert_eq!(
            root.right
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            7
        );
    }
}
