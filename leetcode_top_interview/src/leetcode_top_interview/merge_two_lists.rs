// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut move_node = &mut dummy;
        loop {
            if list1.is_none() {
                move_node.next = list2;
                break;
            }
            if list2.is_none() {
                move_node.next = list1;
                break;
            }
            move_node.next = Self::find_small_and_modify_list(&mut list1, &mut list2);
            move_node = move_node.next.as_mut().unwrap();
        }
        dummy.next.take()
    }
    fn find_small_and_modify_list(
        list1: &mut Option<Box<ListNode>>,
        list2: &mut Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let n1 = list1.as_mut().unwrap();
        let n2 = list2.as_mut().unwrap();
        if n1.val < n2.val {
            let mut tmp = n1.next.take();
            std::mem::swap(list1, &mut tmp);
            tmp
        } else {
            let mut tmp = n2.next.take();
            std::mem::swap(list2, &mut tmp);
            tmp
        }
    }
}
