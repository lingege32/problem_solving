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
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        fn split_list(
            mut head: Option<Box<ListNode>>,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut href = &head;
            let mut len = 0;
            while let Some(h) = href {
                len += 1;
                href = &h.next;
            }
            let step = (len - 1) / 2;
            let mut h_mutref = &mut head;
            for _ in 0..step {
                h_mutref = &mut h_mutref.as_mut().unwrap().next;
            }

            let right_list = h_mutref.as_mut().unwrap().next.take();
            (head, right_list)
        }
        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut dummy = ListNode::new(0);
            while let Some(mut h) = head {
                head = h.next.take();
                h.next = dummy.next.take();
                dummy.next = Some(h);
            }
            dummy.next.take()
        }
        let (mut left, right) = split_list(head.take());
        let mut right = reverse_list(right);
        let mut dummy = ListNode::new(0);
        let mut d = &mut dummy.next;
        while let Some(mut l) = left {
            left = l.next.take();
            *d = Some(l);
            d = &mut d.as_mut().unwrap().next;

            if let Some(mut r) = right {
                right = r.next.take();
                *d = Some(r);
                d = &mut d.as_mut().unwrap().next;
            }
        }
        *head = dummy.next.take();
    }
}
