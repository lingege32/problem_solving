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

struct Solution();
impl Solution {
    #[allow(dead_code)]
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(-1));

        let mut former = head;
        let mut ptr = dummy.as_mut();
        while former.is_some() {
            let mut later = former.as_mut().unwrap().next.take();
            if later.is_none() {
                ptr.next = former;
                break;
            }

            // ptr -/-> former    -/-> later -> next 
            // after swap
            // ptr -/-> next      -/-> later -> former
            std::mem::swap(&mut former, &mut later.as_mut().unwrap().next);
            
            // ptr->later->former -/->next
            ptr.next = later;

            // ptr->later->former //  new ptr    -/->former
            ptr = ptr.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy.next
    }
}
