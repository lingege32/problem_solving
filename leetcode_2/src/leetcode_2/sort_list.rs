// Definition for singly-linked list.
struct Solution {}
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn pop_head(list: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            // list [1, 2, 3]
            // return [1]
            // list [2, 3]
            let mut next = list.as_mut().unwrap().next.take();
            std::mem::swap(&mut next, list);
            next
        }
        fn merge(
            mut lhs: Option<Box<ListNode>>,
            mut rhs: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if lhs.is_none() {
                rhs
            } else if rhs.is_none() {
                lhs
            } else {
                fn take_smallest(
                    lhs: &mut Option<Box<ListNode>>,
                    rhs: &mut Option<Box<ListNode>>,
                ) -> Option<Box<ListNode>> {
                    // take the smallest value ListNode between lhs and rhs, and update the list
                    if lhs.as_ref().unwrap().val < rhs.as_ref().unwrap().val {
                        pop_head(lhs)
                    } else {
                        pop_head(rhs)
                    }
                }
                let mut new_head = take_smallest(&mut lhs, &mut rhs);
                let mut ref_ptr = new_head.as_mut().unwrap();
                while lhs.is_some() && rhs.is_some() {
                    ref_ptr.next = take_smallest(&mut lhs, &mut rhs);
                    ref_ptr = ref_ptr.next.as_mut().unwrap();
                }
                ref_ptr.next = if lhs.is_none() { rhs } else { lhs };
                new_head
            }
        }
        fn merge_sort(mut head: Box<ListNode>) -> Option<Box<ListNode>> {
            let mut split_node: Vec<Option<Box<ListNode>>> = vec![None; 16];
            let mut head = Some(head);
            loop {
                if head.is_none() {
                    break;
                }
                let mut single_head = pop_head(&mut head);
                for idx in 0..16 {
                    if split_node[idx].is_none() {
                        split_node[idx] = single_head;
                        break;
                    } else {
                        single_head = merge(single_head, split_node[idx].take());
                    }
                }
            }
            let mut new_head = split_node[0].take();
            for idx in 1..16 {
                new_head = merge(new_head, split_node[idx].take());
            }
            new_head
        }
        match head {
            Some(head) => merge_sort(head),
            None => None,
        }
    }
}
