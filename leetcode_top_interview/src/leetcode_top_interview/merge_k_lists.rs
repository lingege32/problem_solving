// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;
use std::cmp::Ordering;
impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue = lists
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<std::collections::BinaryHeap<Box<ListNode>>>();
        let mut head = None;
        let mut cur = &mut head;
        while let Some(mut node) = queue.pop() {
            let next = node.next.take();
            *cur = Some(node);
            cur = &mut cur.as_mut().unwrap().next;
            if let Some(n) = next {
                queue.push(n);
            }
        }
        head
    }
}
