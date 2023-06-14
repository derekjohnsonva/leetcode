use leetcode::ListNode;
fn main() {
    println!("Solution to Q876");
}

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut first, mut second) = (&head, &head);
        let mut second_move_flag = false;
        while let Some(val) = first {
            first = &val.next;
            if second_move_flag {
                second = &second.as_ref().unwrap().next;
            }
            second_move_flag = !second_move_flag;
        }
        second.clone()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let head = vec![1, 2, 3, 4, 5];
        let head = leetcode::list_node_from_vec(&head);
        let result = vec![3, 4, 5];
        let res = Solution::middle_node(head);
        assert_eq!(result, leetcode::vec_from_list_node(res));
    }
    #[test]
    fn ex2() {
        let head = vec![1, 2, 3, 4, 5, 6];
        let head = leetcode::list_node_from_vec(&head);
        let result = vec![4, 5, 6];
        let res = Solution::middle_node(head);
        assert_eq!(result, leetcode::vec_from_list_node(res));
    }
}

// Input: head = [1,2,3,4,5]
// Output: [3,4,5]
// Explanation: The middle node of the list is node 3.
//
// Example 2:
//
// Input: head = [1,2,3,4,5,6]
// Output: [4,5,6]
// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
