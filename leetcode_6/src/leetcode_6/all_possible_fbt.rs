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
type RootNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    #[allow(dead_code)]
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut dp: Vec<Option<Vec<RootNode>>> = vec![None; n as usize];
        dp[0] = Some(vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]);
        Self::store_all_binary_tree((n - 1) as usize, &mut dp);
        println!("dp2: {:#?}", dp[2]);
        println!("dp3: {:#?}", dp[3]);
        println!("dp4: {:#?}", dp[4]);
        println!("dp5: {:#?}", dp[5]);
        dp[(n - 1) as usize].take().unwrap()
    }

    fn store_all_binary_tree(idx: usize, dp: &mut Vec<Option<Vec<RootNode>>>) {
        if dp[idx].is_none() {
            let mut all_possible = vec![];
            for one_side_num in 1.. {
                let another_side_num = idx - one_side_num;
                if one_side_num >= another_side_num {
                    break;
                }
                Self::store_all_binary_tree(one_side_num - 1, dp);
                Self::store_all_binary_tree(another_side_num - 1, dp);
                
                let one_side = dp[one_side_num - 1].as_ref().unwrap();
                let ano_side = dp[another_side_num - 1].as_ref().unwrap();
                if one_side.is_empty() || ano_side.is_empty() {
                    continue;
                }
                for one in one_side {
                    if let Some(one_r) = one {
                        for two in ano_side {
                            if let Some(two_r) = two {
                                all_possible.push(Some(Self::create_new_tree(one_r, two_r)));
                                all_possible.push(Some(Self::create_new_tree(two_r, one_r)));
                            }
                        }
                    }
                }
            }
            if idx % 2 == 0 {
                Self::store_all_binary_tree(idx/2 - 1, dp);
                let middle = dp[idx / 2 - 1].as_ref().unwrap();

                if !middle.is_empty() {
                    for (idx, one) in middle.iter().enumerate() {
                        if let Some(o) = one {
                            for (idx2, two) in middle[idx..].iter().enumerate() {
                                if let Some(t) = two {
                                    if idx2!=0 {
                                        all_possible.push(Some(Self::create_new_tree(o, t)));
                                    }
                                    all_possible.push(Some(Self::create_new_tree(t, o)));
                                }
                            }
                        }
                    }
                }
            }
            dp[idx] = Some(all_possible);
        }
    }
    fn create_new_tree(
        left: &Rc<RefCell<TreeNode>>,
        right: &Rc<RefCell<TreeNode>>,
    ) -> Rc<RefCell<TreeNode>> {
        let left_root = left.clone();
        let right_root = right.clone();
        let mut new_root = TreeNode::new(0);
        new_root.left = Some(left_root);
        new_root.right = Some(right_root);
        Rc::new(RefCell::new(new_root))
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 7;
        Solution::all_possible_fbt(n);
    }
}