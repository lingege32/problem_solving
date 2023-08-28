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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {}
    }
    #[allow(dead_code)]
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut dq = VecDeque::new();
        dq.push_back(root);
        let mut serialized_vec = Vec::new();
        while let Some(node_opt) = dq.pop_front() {
            if let Some(node) = node_opt {
                serialized_vec.push(node.borrow().val.to_string());
                dq.push_back(node.borrow().left.clone());
                dq.push_back(node.borrow().right.clone());
            } else {
                serialized_vec.push("#".to_owned());
            }
        }
        while let Some(last) = serialized_vec.last() {
            if last == "#" {
                serialized_vec.pop();
            } else {
                break;
            }
        }
        let serialized = serialized_vec.join(",");
        serialized
    }
    #[allow(dead_code)]
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let serialized_vec = data;
        let serialized_vec: Vec<&str> = serialized_vec.split(",").collect();
        let mut dq: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if serialized_vec.len() == 0 {
            return None;
        }
        let root;
        if let Ok(v) = serialized_vec[0].parse() {
            root = Rc::new(RefCell::new(TreeNode {
                val: v,
                left: None,
                right: None,
            }));
        } else {
            return None;
        }
        let serialized_vec = &serialized_vec[1..];
        dq.push_back(root.clone());
        // println!("{:?}", serialized_vec);
        for val_serialized in serialized_vec.chunks(2) {
            if let Some(parent) = dq.pop_front() {
                if val_serialized[0] == "#" {
                } else {
                    parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                        val: val_serialized[0].parse::<i32>().unwrap(),
                        left: None,
                        right: None,
                    })));
                    dq.push_back(parent.borrow().left.clone().unwrap());
                }
                if val_serialized.len() > 1 {
                    if val_serialized[1] == "#" {
                    } else {
                        parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                            val: val_serialized[1].parse::<i32>().unwrap(),
                            left: None,
                            right: None,
                        })));
                        dq.push_back(parent.borrow().right.clone().unwrap());
                    }
                }
            } else {
                break;
            }
        }
        return Some(root);
    }
}
