pub struct Solution;
/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, 
and each of their nodes contains a single digit. 
Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

 

Example 1:


Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.
Example 2:

Input: l1 = [0], l2 = [0]
Output: [0]
Example 3:

Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]
 

Constraints:

The number of nodes in each linked list is in the range [1, 100].
0 <= Node.val <= 9
It is guaranteed that the list represents a number that does not have leading zeros.
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut t = (l1, l2, 0, 0);
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, _) => (None, None, 1, 0), 
                (Some(list), None, _, 0) | (None, Some(list), _, 0)
                    => (list.next, None, list.val, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry)
                    if list.val + carry >= 10 => (list.next, None, list.val + 1 - 10, 1),
                (Some(list), None, _, carry) | (None, Some(list), _, carry)
                    => (list.next, None, list.val + carry, 0),
                (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10
                    => (l1.next, l2.next, l1.val + l2.val + carry - 10, 1),
                (Some(l1), Some(l2), _, carry)
                    => (l1.next, l2.next, l1.val + l2.val + carry, 0)
            };
            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = &mut tail.as_mut().unwrap().next; 
        }
        return head;
    }

    pub fn print(node: Option<Box<ListNode>>) {
        let mut tmp = node;
        loop {
            if let Some(n) = tmp {
                print!("{} -> ", n.val);
                tmp = n.next;
            } else {
                println!("end");
                return
            }
        }
    }

}


#[cfg(test)]
mod tests{

    use super::ListNode;
    use super::Solution;


    #[test]
    fn test_print() {
        let mut first = Box::new(ListNode::new(1));
        first.next = Some(Box::new(ListNode::new(2)));
        let mut tmp = &mut first.next;
        for i in  3..=100 {
            let new_node = Box::new(ListNode::new(i));
            if let Some(ref mut n)  = tmp {
                n.next = Some(new_node);
                tmp = &mut n.next
            }
        }
        Solution::print(Some(first));
    }

    #[test]
    fn test_ex1() {
        let mut l11 = Box::new(ListNode::new(2));
        let mut l12 = Box::new(ListNode::new(3));
        let l13 = Box::new(ListNode::new(4));
        l12.next = Some(l13);
        l11.next = Some(l12);
        let l1 = Some(l11);
        let mut l21 = ListNode::new(5);
        let mut l22 = ListNode::new(6);
        let l23 = ListNode::new(6);
        l22.next = Some(Box::new(l23));
        l21.next = Some(Box::new(l22));
        let l2 = Some(Box::new(l21));
        let result = Solution::add_two_numbers(l1, l2);
        Solution::print(result);
    }
}
