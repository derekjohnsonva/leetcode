use leetcode::ListNode;
fn main() {
    println!("Solution to Q206");
}

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur_node = head;
        let mut prev_node: Option<Box<ListNode>> = None;
        while let Some(mut node) = cur_node {
            let next_copy = node.next;
            match prev_node {
                Some(prev_node_v) => node.next = Some(Box::clone(&prev_node_v)),
                None => node.next = None,
            }
            prev_node = Some(Box::clone(&node));
            cur_node = next_copy;

        }
        prev_node
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ex1() {
        let vals = vec![1, 2, 3];
        let ln = leetcode::list_node_from_vec(&vals);
        assert_eq!(ln.clone().unwrap().val, vals[0]);
        assert_eq!(ln.clone().unwrap().next.unwrap().val, vals[1]);
        assert_eq!(ln.clone().unwrap().next.unwrap().next.unwrap().val, vals[2]);
        assert_eq!(ln.clone().unwrap().next.unwrap().next.unwrap().next, None);

        let vals_reconstructed = leetcode::vec_from_list_node(ln);
        assert_eq!(vals_reconstructed, vals);
    }

    #[test]
    fn ex2() {
        let mut vals = vec![1, 2];
        let head = leetcode::list_node_from_vec(&vals);
        let reversed_head = Solution::reverse_list(head);
        vals.reverse();
        assert_eq!(leetcode::vec_from_list_node(reversed_head), vals);
    }
    #[test]
    fn ex3() {
        let mut vals = vec![1, 2, 3, 4, 5, 6];
        let head = leetcode::list_node_from_vec(&vals);
        let reversed_head = Solution::reverse_list(head);
        vals.reverse();
        assert_eq!(leetcode::vec_from_list_node(reversed_head), vals);
    }
    #[test]
    fn ex4() {
        let mut vals = vec![];
        let head = leetcode::list_node_from_vec(&vals);
        let reversed_head = Solution::reverse_list(head);
        vals.reverse();
        assert_eq!(leetcode::vec_from_list_node(reversed_head), vals);
    }
}

// Input: head = [1,2]
// Output: [2,1]
//
// Example 3:
//
// Input: head = []
// Output: []
