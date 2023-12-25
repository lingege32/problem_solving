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
const BIT: usize = 16;
impl Solution {
    #[allow(dead_code)]
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr: Vec<Option<Box<ListNode>>> = vec![None; BIT];
        while let Some(mut h) = head {
            head = h.next.take();
            let mut h = Some(h);
            for idx in 0..BIT {
                let sorted_list = arr[idx].take();
                match sorted_list {
                    Some(list) => {
                        h = Self::merge_list(h, Some(list));
                    }
                    None => {
                        arr[idx] = h;
                        break;
                    }
                }
            }
        }
        arr.iter_mut()
            .filter(|node| node.is_some())
            .reduce(|acc, node| {
                *acc = Self::merge_list(acc.take(), node.take());
                acc
            })
            .unwrap_or(&mut None)
            .take()
    }
    fn merge_list(
        mut left: Option<Box<ListNode>>,
        mut right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut r = &mut head;
        loop {
            if left.is_none() {
                *r = right;
                break;
            }
            if right.is_none() {
                *r = left;
                break;
            }

            let l_val = left.as_ref().unwrap().val;
            let r_val = right.as_ref().unwrap().val;
            *r = if l_val < r_val {
                let mut new_node = left.as_mut().unwrap().next.take();
                std::mem::swap(&mut left, &mut new_node);
                new_node
            } else {
                let mut new_node = right.as_mut().unwrap().next.take();
                std::mem::swap(&mut right, &mut new_node);
                new_node
            };

            r = &mut r.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut n1 = Box::new(ListNode::new(4));
        let mut n2 = Box::new(ListNode::new(2));
        let mut n3 = Box::new(ListNode::new(1));
        let n4 = Box::new(ListNode::new(3));
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);
        let a = Solution::sort_list(Some(n1));
        println!("{:?}", a);
    }
    #[test]
    fn test_2() {
        let mut n1 = Box::new(ListNode::new(-1));
        let mut n2 = Box::new(ListNode::new(5));
        let mut n3 = Box::new(ListNode::new(3));
        let mut n4 = Box::new(ListNode::new(4));
        let n5 = Box::new(ListNode::new(0));
        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);
        let a = Solution::sort_list(Some(n1));
        println!("{:?}", a);
    }
}
