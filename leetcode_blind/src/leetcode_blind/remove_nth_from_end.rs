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
impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut head = head;
        dummy.next = head.take();
        let mut cur = &dummy;
        for _ in 0..n {
            cur = cur.next.as_ref().unwrap();
        }
        let mut left = &dummy;
        while let Some(c) = cur.next.as_ref() {
            cur = c;
            left = left.next.as_ref().unwrap();
        }
        unsafe {
            let left = left as *const ListNode;
            let left = left as *mut ListNode;
            // let left = &mut *left;
            let mut n = (*left).next.take().unwrap();
            (*left).next = n.next.take();
        }

        dummy.next.take()
    }
}
