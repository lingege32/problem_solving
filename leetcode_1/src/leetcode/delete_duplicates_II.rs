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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            None
        } else {
            let mut prev = Some(Box::new(ListNode::new(0)));
            prev.as_mut().unwrap().next = head;
            let mut pr = prev.as_mut().unwrap();
            while let Some(cur) = pr.next.as_mut() {
                if cur.next.as_ref().map_or(false, |x| x.val == cur.val) {
                    let c = pr.next.take();
                    pr.next = Self::remove_head_duplicate(c);
                } else {
                    pr = pr.next.as_mut().unwrap();
                }
            }

            prev.unwrap().next
        }
    }
    fn remove_head_duplicate(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // the assumption is that the first n elements value are equal, n>1;
        if head.is_none() {
            None
        } else {
            let mut head = head;
            let v = head.as_ref().unwrap().val;
            while let Some(h) = head.as_mut() {
                if h.val != v {
                    break;
                }
                head = h.next.take();
            }
            head
        }
    }
}
