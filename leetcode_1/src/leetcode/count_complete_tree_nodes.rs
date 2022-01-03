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
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            Self::recursive_check(r, None, None)
        } else {
            0
        }
    }
    fn recursive_check(
        root: Rc<RefCell<TreeNode>>,
        left_deep: Option<i32>,
        right_deep: Option<i32>,
    ) -> i32 {
        let ld = left_deep.unwrap_or_else(|| Self::check_left_deep(&root));
        let rd = right_deep.unwrap_or_else(|| Self::check_right_deep(&root));
        println!("root: {}, left: {}, right: {}", root.borrow().val, ld, rd);
        if ld == rd {
            i32::pow(2, ld as u32) - 1
        } else if rd == 1 {
            2
        } else {
            let new_ld = Some(ld - 1);
            let new_rd = Some(rd - 1);
            let left_total =
                Self::recursive_check(root.borrow().left.clone().unwrap(), new_ld, None);
            let right_total =
                Self::recursive_check(root.borrow().right.clone().unwrap(), None, new_rd);
            left_total + 1 + right_total
        }
    }
    fn check_left_deep(root: &Rc<RefCell<TreeNode>>) -> i32 {
        let b = root.borrow();
        match b.left {
            Some(ref l) => 1 + Self::check_left_deep(l),
            None => 1,
        }
    }
    fn check_right_deep(root: &Rc<RefCell<TreeNode>>) -> i32 {
        let b = root.borrow();
        match b.right {
            Some(ref l) => 1 + Self::check_right_deep(l),
            None => 1,
        }
    }
}

// other's 100% Solutionuse std::cell::RefCell;
// impl Solution {
//     pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         Self::count_nodes_recursive(&root, None, None)
//     }
//     fn count_nodes_recursive(
//         node: &Option<Rc<RefCell<TreeNode>>>,
//         known_left_depth: Option<i32>,
//         known_right_depth: Option<i32>,
//     ) -> i32 {
//         let left_depth = known_left_depth.unwrap_or_else(|| Self::probe_left_depth(node));
//         let right_depth = known_right_depth.unwrap_or_else(|| Self::probe_right_depth(node));
//         if left_depth == right_depth {
//             Self::volume_from_height(left_depth)
//         } else if let Some(node) = node {
//             let borrowed = node.borrow();
//             let left_count =
//                 Self::count_nodes_recursive(&borrowed.left, Some(left_depth - 1), None);
//             let right_count =
//                 Self::count_nodes_recursive(&borrowed.right, None, Some(right_depth - 1));
//             left_count + 1 + right_count
//         } else {
//             0
//         }
//     }
//     fn probe_left_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         if let Some(node) = node {
//             Self::probe_left_depth(&node.borrow().left) + 1
//         } else {
//             0
//         }
//     }
//     fn probe_right_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         if let Some(node) = node {
//             Self::probe_right_depth(&node.borrow().right) + 1
//         } else {
//             0
//         }
//     }
//     fn volume_from_height(n: i32) -> i32 {
//         i32::pow(2, n as u32) - 1
//     }
// }


// my BFS solution
// use std::cell::RefCell;
// use std::collections::VecDeque;
// use std::rc::Rc;
// impl Solution {
//     pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut result = 0;
//         if let Some(r) = root {
//             let mut queue = VecDeque::new();
//             queue.push_back(r);
//             while !queue.is_empty() {
//                 let len = queue.len();
//                 result += len;
//                 for _ in 0..len {
//                     let node = queue.pop_front().unwrap();
//                     let option_left = node.borrow().left.clone();

//                     if let Some(left) = option_left {
//                         queue.push_back(left.clone());
//                     }
//                     let option_right = node.borrow().right.clone();

//                     if let Some(right) = option_right {
//                         queue.push_back(right.clone());
//                     }
//                 }
//             }
//         }
//         result as i32
//     }
// }