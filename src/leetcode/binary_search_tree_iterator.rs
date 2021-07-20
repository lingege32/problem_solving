// other's better Solution// Definition for a binary tree node.
// do not traverse all tree in the new function
// only store the left child to the stack
// and when we call next, we pop out the last one and check if the right node is existed in the parent.
// if yes, add the all left node to the stack.
//
//
//
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

// use std::cell::RefCell;
// use std::rc::Rc;

// type Node = Rc<RefCell<TreeNode>>;


// struct BSTIterator {
//     stack: Vec<Node>
// }


// /** 
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl BSTIterator {

//     fn new(root: Option<Node>) -> Self {
//         let mut this = Self { stack: Vec::new() };
//         this.add_left_line(root);
//         this
//     }
    
//     fn add_left_line(&mut self, root: Option<Node>) {
//         let mut mnode = root;
//         while let Some(node) = mnode {
//             self.stack.push(node.clone());
//             mnode = node.borrow().left.clone();
//         }
//     }
    
//     fn next(&mut self) -> i32 {
//         let node = self.stack.pop().unwrap();
//         self.add_left_line(node.borrow().right.clone());
//         let x = node.borrow().val; x
//     }
    
//     fn has_next(&mut self) -> bool {
//         !self.stack.is_empty()
//     }
// }











































// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::{cell::RefCell, collections::LinkedList, rc::Rc};
struct BSTIterator {
    queue: LinkedList<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, queue: &mut LinkedList<i32>) {
            if let Some(r) = root {
                dfs(r.borrow().left.clone(), queue);
                queue.push_back(r.borrow().val);
                dfs(r.borrow().right.clone(), queue);
            }
        }
        let mut q = LinkedList::new();
        dfs(root, &mut q);
        BSTIterator {
            queue: q
        }
    }
    
    fn next(&mut self) -> i32 {
        if self.has_next() {
            self.queue.pop_front().unwrap()
        } else {
            0
        }
    }
    
    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}
