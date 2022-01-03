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

// this easy solution is faster........
// fuckkkkkkkk
// impl Solution {
//     pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
//         let mut v = vec![];
//         while let Some(a) = head {
//             v.push(a.val);
//             head = a.next;
//         }
//         for i in 0..v.len() / 2{
//             if v.get(i) != v.get(v.len() - i -1) {return false}
//         }
//         true
//     }
// }



struct Solution;
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return true;
        }
        let mut node = &head;
        let mut len = 0;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        let mid = len / 2;
        let mut node = &mut head;
        for _ in 0..mid - 1 {
            node = &mut node.as_mut().unwrap().next;
        }
        let h2 = node.as_mut().unwrap().next.take();

        fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut dummy = ListNode::new(0);
            dummy.next = head;
            let mut prev = None;
            while let Some(mut n) = dummy.next {
                dummy.next = n.next.take();
                n.next = prev;
                prev = Some(n);
            }
            prev
        }

        let mut rev_h2 = reverse_list(h2);
        while let Some(n1) = head {
            let n2 = rev_h2.unwrap();
            if n1.val != n2.val {
                return false;
            }
            head = n1.next;
            rev_h2 = n2.next;
        }

        true
    }
}
