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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        let mut dummy = {
            let mut d = ListNode::new(0);
            d.next = head;
            Some(Box::new(d))
        };
        let mut cur = &mut dummy;
        for _ in 0..left - 1 {
            cur = &mut cur.as_mut().unwrap().next;
        }

        /*
         * Split the linked list into three part
         * 1->2->3->4->5 to dummy->1->2->3->4->5
         * if left = 2 and right = 4.
         * dummy is dummy->1->2
         * cur is referenct to the last of dummy.
         * head is 3->4
         * third_part is 5
         */
        let mut head = cur.as_mut().unwrap().next.take();
        let mut second_cur = &mut head;
        for _ in 0..right - left {
            second_cur = &mut second_cur.as_mut().unwrap().next;
        }
        let mut third_part = second_cur.as_mut().unwrap().next.take();
        loop {
            // head must have value
            let mut h = head.unwrap();
            head = h.next.take();

            h.next = third_part;
            third_part = Some(h);

            if head.is_none() {
                cur.as_mut().unwrap().next = third_part;
                break;
            }
        }
        dummy.unwrap().next.take()
    }
}
