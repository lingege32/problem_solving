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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = head;

        loop {
            if curr.is_none() {
                break;
            } else {
                let mut left = dummy_head.as_mut();
                while left.as_ref().unwrap().next.is_some()
                    && left.as_ref().unwrap().next.as_ref().unwrap().val < curr.as_ref().unwrap().val
                {
                    left = left.unwrap().next.as_mut();
                }
                let right = left.as_mut().unwrap().next.take();
                let next = curr.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = right;
                left.as_mut().unwrap().next = curr;
                curr = next;
            }
        }
        dummy_head.unwrap().next
    }
}

// impl Solution {
//     #[allow(dead_code)]
//     pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         // practice merge sort of c++
//         fn merge(
//             mut list1: Option<Box<ListNode>>,
//             mut list2: Option<Box<ListNode>>,
//         ) -> Option<Box<ListNode>> {
//             let mut tmp = ListNode::new(0);
//             let mut head = &mut tmp;
//             loop {
//                 if list1.is_none() {
//                     head.next = list2;
//                     break;
//                 }
//                 if list2.is_none() {
//                     head.next = list1;
//                     break;
//                 }
//                 if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
//                     head.next = list1;
//                     head = head.next.as_mut().unwrap();
//                     list1 = head.next.take();
//                 } else {
//                     head.next = list2;
//                     head = head.next.as_mut().unwrap();
//                     list2 = head.next.take();
//                 }
//             }
//             tmp.next.take()
//         }
//         let mut buffer: [Option<Box<ListNode>>; 14] = Default::default();
//         let mut head = head;
//         while head.is_some() {
//             let mut h = head;
//             head = h.as_mut().unwrap().next.take();
//             for idx in 0..14 {
//                 let inbuffer = buffer[idx].take();
//                 match inbuffer {
//                     Some(b) => {
//                         buffer[idx] = merge(Some(b), h.take());
//                     }
//                     None => {
//                         buffer[idx] = h.take();
//                     }
//                 }
//             }
//         }
//         let mut ans = None;
//         for idx in 0..14 {
//             ans = merge(ans, buffer[idx].take());
//         }
//         ans
//     }
// }
