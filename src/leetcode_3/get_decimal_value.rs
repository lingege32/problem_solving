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
    #[allow(dead_code)]
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut ans = 0;
        while let Some(n) = head {
            ans = ans * 2 + n.val;
            head = n.next;
        }
        ans
    }
}
