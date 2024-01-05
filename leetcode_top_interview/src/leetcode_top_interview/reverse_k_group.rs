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
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        for _ in 1..k {
            match cur {
                Some(c) => {
                    cur = &mut c.next;
                }
                None => {
                    break;
                }
            }
        }
        match cur {
            Some(c) => {
                let next = Self::reverse_k_group(c.next.take(), k);
                let mut new_head = Self::reverse(head);
                let mut cur = &mut new_head;
                for _ in 1..k {
                    cur = &mut cur.as_mut().unwrap().next;
                }
                cur.as_mut().unwrap().next = next;
                new_head
            }
            None => head,
        }
    }

    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        while let Some(mut h) = head {
            head = h.next.take();
            h.next = dummy.next.take();
            dummy.next = Some(h);
        }
        dummy.next.take()
    }
}
