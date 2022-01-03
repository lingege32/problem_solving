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






// solution is reference from https://zxi.mytechroad.com/blog/tree/leetcode-99-recover-binary-search-tree/
struct Solution {
    
}
use std::rc::Rc;
use std::cell::RefCell;
struct Ans {
    prev : Option<Rc<RefCell<TreeNode>>> ,
    first : Option<Rc<RefCell<TreeNode>>> ,
    second : Option<Rc<RefCell<TreeNode>>> ,
}
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_some() {
            let mut ans = Ans {
                prev: None,
                first: None,
                second: None,
            };
            Self::inorder(root.clone(), &mut ans);
            std::mem::swap(&mut ans.first.unwrap().borrow_mut().val, &mut ans.second.unwrap().borrow_mut().val);
        }
    }
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Ans) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        Self::inorder(root.borrow().left.clone(), ans);
        if let Some(prev_node) = &ans.prev {
            if prev_node.borrow().val > root.borrow().val {
                if ans.first.is_none() {
                    ans.first = Some(prev_node.clone());
                }
                ans.second = Some(root.clone());
            }
        }
        ans.prev = Some(root.clone());
        Self::inorder(root.borrow().right.clone(), ans);

    }
}