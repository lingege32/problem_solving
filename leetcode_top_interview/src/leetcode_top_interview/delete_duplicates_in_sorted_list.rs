// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut dummy = ListNode { val: 0, next: None };
        let mut cur = &mut dummy.next;
        let mut cur_val = i32::MAX;

        while let Some(mut node) = head {
            head = node.next.take();
            if cur_val != node.val {
                cur_val = node.val;
                if cur_val != head.as_ref().map(|o| o.val).unwrap_or(i32::MAX) {
                    *cur = Some(node);
                    cur = &mut cur.as_mut().unwrap().next;
                }
            }
        }
        dummy.next.take()
    }
}
