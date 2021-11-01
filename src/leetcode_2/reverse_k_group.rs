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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();
        'outer: loop {
            let mut start = head.as_mut().unwrap().next.take();
            if start.is_none() {
                break 'outer;
            }
            let mut end = start.as_mut();
            for _ in 0..(k - 1) {
                end = end.unwrap().next.as_mut();
                if end.is_none() {
                    head.as_mut().unwrap().next = start;
                    break 'outer;
                }
            }
            let mut tail = end.as_mut().unwrap().next.take();
            // BEFORE: head -> start -> 123456... -> end   -> tail
            // AFTER:  head -> end   -> ...654321 -> start -> tail
            let end = Solution::reverse(start, tail);
            head.as_mut().unwrap().next = end;
            for _ in 0..k {
                head = head.unwrap().next.as_mut()
            }
        }
        dummy_head.unwrap().next
    }

    #[inline(always)]
    fn reverse(
        mut head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = tail;
        let mut current = head;
        while let Some(mut current_node_inner) = current {
            let mut next = current_node_inner.next.take();
            current_node_inner.next = prev.take();
            prev = Some(current_node_inner);
            current = next;
        }
        prev
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n5 = Box::new(ListNode::new(5));
        let mut n4 = Box::new(ListNode::new(4));
        let mut n3 = Box::new(ListNode::new(3));
        let mut n2 = Box::new(ListNode::new(2));
        let mut n1 = Box::new(ListNode::new(1));
        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);

        let mut h = Solution::reverse_k_group(Some(n1), 2);
        let mut v = Vec::new();
        while let Some(n) = h {
            v.push(n.val);
            h = n.next;
        }
        assert_eq!(v, vec![2, 1, 4, 3, 5]);
    }
}
