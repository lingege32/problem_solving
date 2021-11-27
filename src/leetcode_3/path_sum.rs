// Definition for a binary tree node.

struct Solution {}
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
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn inner(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            current_sum: i32,
            map: &mut HashMap<i32, i32>,
            count: &mut i32,
        ) {
            if let Some(r) = root {
                let val = r.borrow().val;
                let new_sum = current_sum + val;
                let need_sum = new_sum - target_sum;
                if let Some(c) = map.get(&need_sum) {
                    *count += *c;
                }
                *map.entry(new_sum).or_insert(0) += 1;
                inner(r.borrow_mut().left.take(), target_sum, new_sum, map, count);
                inner(r.borrow_mut().right.take(), target_sum, new_sum, map, count);
                *map.entry(new_sum).or_insert(0) -= 1;
            }
        }
        let mut map = HashMap::new();
        map.insert(0, 1);
        let mut count = 0;
        inner(root, target_sum, 0, &mut map, &mut count);
        count
    }
}

// val: 5, current_sum: 0, new_sum: 5, need_sum: -17, map: {}
// val: 4, current_sum: 5, new_sum: 9, need_sum: -13, map: {5: 1}
// val: 11, current_sum: 9, new_sum: 20, need_sum: -2, map: {5: 1, 9: 1}
// val: 7, current_sum: 20, new_sum: 27, need_sum: 5, map: {5: 1, 9: 1, 20: 1}
// val: 2, current_sum: 20, new_sum: 22, need_sum: 0, map: {5: 1, 20: 1, 9: 1, 27: 0}
// val: 8, current_sum: 5, new_sum: 13, need_sum: -9, map: {5: 1, 20: 0, 9: 0, 27: 0, 22: 0}
// val: 13, current_sum: 13, new_sum: 26, need_sum: 4, map: {5: 1, 13: 1, 20: 0, 9: 0, 27: 0, 22: 0}
// val: 4, current_sum: 13, new_sum: 17, need_sum: -5, map: {5: 1, 26: 0, 13: 1, 20: 0, 9: 0, 27: 0, 22: 0}
// val: 5, current_sum: 17, new_sum: 22, need_sum: 0, map: {20: 0, 27: 0, 26: 0, 5: 1, 13: 1, 17: 1, 9: 0, 22: 0}
// val: 1, current_sum: 17, new_sum: 18, need_sum: -4, map: {20: 0, 27: 0, 26: 0, 5: 1, 13: 1, 17: 1, 9: 0, 22: 0}
