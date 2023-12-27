// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;
type NodePtr = Option<Node>;
struct BSTIterator {
    m_stack: Vec<Node>,
}

impl BSTIterator {
    #[allow(dead_code)]
    fn new(root: NodePtr) -> Self {
        let mut ret = Self { m_stack: vec![] };
        ret.push_all(root);
        ret
    }

    #[allow(dead_code)]
    fn next(&mut self) -> i32 {
        if let Some(n) = self.m_stack.pop() {
            self.push_all(n.borrow().right.clone());
            return n.borrow().val;
        }

        std::unreachable!()
    }

    #[allow(dead_code)]
    fn has_next(&self) -> bool {
        !self.m_stack.is_empty()
    }

    #[allow(dead_code)]
    fn push_all(&mut self, mut node: NodePtr) {
        while let Some(n) = node {
            self.m_stack.push(n.clone());
            node = n.borrow().left.clone();
        }
    }
}
