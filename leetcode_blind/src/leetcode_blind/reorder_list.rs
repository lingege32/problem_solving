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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let new_head = head.take();
        let (l1, l2) = Self::split(new_head);
        let l2 = Self::reverse(l2);
        *head = Self::merge(l1, l2);
    }
    fn push(node: &mut ListNode, mut next_list: Box<ListNode>) -> Option<Box<ListNode>> {
        let ret = next_list.next.take();
        node.next = Some(next_list);
        ret
    }
    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut dummy = ListNode::new(0);
        dummy.next = head.take();
        let mut cur = &mut dummy;
        let mut len = 0;
        while let Some(n) = cur.next.as_mut() {
            len += 1;
            cur = n;
        }

        let list1_len = (len + 1) / 2;
        let mut cur = &mut dummy;
        for _ in 0..list1_len {
            cur = cur.next.as_mut().unwrap();
        }
        let l2 = cur.next.take();
        (dummy.next.take(), l2)
    }
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        while let Some(mut h) = head {
            let next = h.next.take();
            h.next = dummy.next.take();
            dummy.next = Some(h);
            head = next;
        }
        dummy.next.take()
    }
    fn merge(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        loop {
            let (h, n) = Self::extract_head(list1);
            if h.is_some() {
                list1 = n;
                cur.next = h;
                cur = cur.next.as_mut().unwrap();
            } else {
                break;
            }
            let (h, n) = Self::extract_head(list2);
            if h.is_some() {
                list2 = n;
                cur.next = h;
                cur = cur.next.as_mut().unwrap();
            } else {
                break;
            }
        }

        dummy.next.take()
    }
    fn extract_head(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        match head {
            Some(mut h) => {
                let next = h.next.take();
                (Some(h), next)
            }
            None => (None, None),
        }
    }
}
