// Definition for singly-linked list.
struct Solution;
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(1);
        let mut cur = &mut dummy;
        let mut list1 = list1;
        let mut list2 = list2;
        loop {
            if list1.is_none() {
                cur.next = list2;
                break;
            }
            if list2.is_none() {
                cur.next = list1;
                break;
            }
            let v1 = list1.as_ref().unwrap().as_ref().val;
            let v2 = list2.as_ref().unwrap().as_ref().val;
            if v1 < v2 {
                let mut tmp = list1.unwrap();
                list1 = tmp.next.take();
                cur.next = Some(tmp);
                cur = cur.next.as_mut().unwrap();
            } else {
                let mut tmp = list2.unwrap();
                list2 = tmp.next.take();
                cur.next = Some(tmp);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy.next.take()
    }
}
