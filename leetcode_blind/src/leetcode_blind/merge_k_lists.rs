struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());
        for list in lists {
            match list {
                Some(list) => heap.push(list),
                None => {}
            }
        }
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        while heap.len() != 0 {
            let mut node = heap.pop().unwrap();
            let n = node.next.take();
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
            if let Some(nn) = n {
                heap.push(nn);
            }
        }
        dummy.next.take()
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
