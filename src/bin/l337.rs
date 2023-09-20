pub struct Solution {}

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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn dp(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = node {
            let (left_take, left_remain) = Self::dp(&node.borrow().left);
            let (right_take, right_remain) = Self::dp(&node.borrow().right);
            let take = node.borrow().val + left_remain + right_remain;
            let remain =
                std::cmp::max(left_take, left_remain) + std::cmp::max(right_take, right_remain);
            return (take, remain);
        }
        (0, 0)
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (take, remain) = Self::dp(&root);
        std::cmp::max(take, remain)
    }
}

pub fn main() {}