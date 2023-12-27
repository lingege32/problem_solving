// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(0);
        let mut dummy2 = ListNode::new(0);
        let mut left = &mut dummy1.next;
        let mut right = &mut dummy2.next;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                *left = Some(node);
                left = &mut left.as_mut().unwrap().next;
            } else {
                *right = Some(node);
                right = &mut right.as_mut().unwrap().next;
            }
        }

        *left = dummy2.next.take();
        dummy1.next.take()
    }
}
