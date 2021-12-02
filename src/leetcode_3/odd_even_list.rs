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
// more clearly code  
impl Solution {
    #[allow(dead_code)]
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut odd_dummy = ListNode::new(0);
        let mut even_dummy = ListNode::new(0);
        let mut odd_curr = &mut odd_dummy;
        let mut even_curr = &mut even_dummy;

        let mut odd = true;
        while let Some(mut node) = head {
            head = node.next.take();
            if odd {
                odd_curr.next = Some(node);
                odd_curr = odd_curr.next.as_mut().unwrap();
            } else {
                even_curr.next = Some(node);
                even_curr = even_curr.next.as_mut().unwrap();
            }
            odd = !odd;
        }
        odd_curr.next = even_dummy.next;
        odd_dummy.next
    }
}

// impl Solution {
//     #[allow(dead_code)]
//     pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         fn take_next(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//             match node {
//                 Some(b) => b.next.take(),
//                 None => None,
//             }
//         }
//         if head.is_none() {
//             head
//         } else {
//             let mut odd = head;
//             let mut even = take_next(&mut odd);
//             if even.is_none() {
//                 odd
//             } else {
//                 let mut retain = take_next(&mut even);
//                 let mut odd_ref = odd.as_mut().unwrap();
//                 let mut even_ref = even.as_mut().unwrap();
//                 loop {
//                     if retain.is_some() {
//                         let tmp = take_next(&mut retain);
//                         odd_ref.next = retain;
//                         retain = tmp;
//                         odd_ref = odd_ref.next.as_mut().unwrap();
//                     } else {
//                         break;
//                     }
//                     if retain.is_some() {
//                         let tmp = take_next(&mut retain);
//                         even_ref.next = retain;
//                         retain = tmp;
//                         even_ref = even_ref.next.as_mut().unwrap();
//                     } else {
//                         break;
//                     }
//                 }
//                 odd_ref.next = even;
//                 odd
//             }
//         }
//     }
// }
