// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::inner(&nums)
    }
    fn inner(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        match len {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(nums[0])))),
            l => {
                let m = l / 2;
                let (left, right) = nums.split_at(m);
                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[m],
                    left: Self::inner(left),
                    right: Self::inner(&right[1..]),
                })))
            }
        }
    }
}
