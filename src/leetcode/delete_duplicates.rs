// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution {
    
}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            None
        } else {
            let mut head = head;
            let mut h = head.as_mut();
            loop {
                let n = h.unwrap();
                if n.next.is_none() {
                    break;
                }
                if n.val==n.next.as_ref().unwrap().val {
                    let new = n.next.take();
                    n.next = new.unwrap().next;
                    h = Some(n);
                } else {
                    h = n.next.as_mut();
                }
            }
            head
        }
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = ListNode::new(1);
        let mut b= ListNode::new(1);
        let mut b1= ListNode::new(2);
        let mut b2= ListNode::new(3);
        let c = ListNode::new(3);
        b2.next = Some(Box::new(c));
        b1.next = Some(Box::new(b2));
        b.next = Some(Box::new(b1));
        a.next = Some(Box::new(b));
        println!("{:?}", Solution::delete_duplicates(Some(Box::new(a))));
        assert!(true);

    }
}