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
            right: None,
        }
    }
}
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::inner(root, target_sum, &mut res);
        for v in res.iter_mut() {
            v.reverse();
        }
        res
    }
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, res_vec: &mut Vec<Vec<i32>>)  {
        if let Some(r) = root {
            fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
                node.borrow().left.is_none() && node.borrow().right.is_none()
            }
            if is_leaf(&r) {
                if r.borrow().val == target_sum {
                    res_vec.push(vec![target_sum]);
                }
            } else {
                let val = r.borrow().val;
                let cur_size = res_vec.len();
                Self::inner(r.borrow().left.clone(), target_sum - val, res_vec);
                Self::inner(r.borrow().right.clone(), target_sum - val, res_vec);
                for v in &mut res_vec[cur_size..] {
                    v.push(val);
                }
            }
        }
    }
}
