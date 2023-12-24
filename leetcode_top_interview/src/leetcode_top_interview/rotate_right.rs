// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
struct Solution;
impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let len = Self::list_len(&head);
        let k = k % len;
        let j = len - k;
        let mut cur = &mut head;
        for _ in 0..j {
            cur = &mut cur.as_mut().unwrap().next;
        }
        let mut new_head = cur.take();
        let mut new_head_cur = &mut new_head;
        while new_head_cur.is_some() {
            new_head_cur = &mut new_head_cur.as_mut().unwrap().next;
        }
        *new_head_cur = head;
        new_head
    }
    fn list_len(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;
        while let Some(n) = head {
            ans += 1;
            head = &n.next;
        }
        ans
    }
}
