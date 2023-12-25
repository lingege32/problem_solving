// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
struct Solution;
impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = Self::cal_len(&head);

        let mut ref_to_node = &mut head;
        for _ in 0..len - n as usize {
            ref_to_node = &mut ref_to_node.as_mut().unwrap().next;
        }
        *ref_to_node = ref_to_node.as_mut().unwrap().next.take();

        head
    }

    fn cal_len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut ans = 0;
        while let Some(h) = head {
            ans += 1;
            head = &h.next;
        }
        ans
    }
}
