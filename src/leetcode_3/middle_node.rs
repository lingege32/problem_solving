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
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h = &head;
        let mut len = 0;
        while let Some(n) = h {
            len+=1;
            h=&n.next;
        }
        let step = len/2;
        let mut head = head;
        for _ in 0..step {
            head = head.unwrap().next;
        }
        head
    }
}