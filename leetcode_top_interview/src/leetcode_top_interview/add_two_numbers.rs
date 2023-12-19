// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;
impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::inner(l1, l2, 0)
    }
    fn inner(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match l1 {
            Some(mut n1) => match l2 {
                Some(mut n2) => {
                    let new_val = n1.val + n2.val + carry;
                    let mut new_node = Box::new(ListNode::new(new_val % 10));
                    new_node.next = Self::inner(n1.next.take(), n2.next.take(), new_val / 10);
                    Some(new_node)
                }
                None => {
                    let new_val = n1.val + carry;
                    let mut new_node = Box::new(ListNode::new(new_val % 10));
                    new_node.next = Self::inner(n1.next.take(), None, new_val / 10);
                    Some(new_node)
                }
            },
            None => match l2 {
                Some(mut n2) => {
                    let new_val = n2.val + carry;
                    let mut new_node = Box::new(ListNode::new(new_val % 10));
                    new_node.next = Self::inner(None, n2.next.take(), new_val / 10);
                    Some(new_node)
                }
                None => {
                    if carry > 0 {
                        Some(Box::new(ListNode::new(carry)))
                    } else {
                        None
                    }
                }
            },
        }
    }
}
