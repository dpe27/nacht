use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal_94(_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        vec![]
    }

    pub fn is_same_tree_100(
        _p: Option<Rc<RefCell<TreeNode>>>,
        _q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        false
    }

    pub fn is_symmetric_101(_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        false
    }

    pub fn max_depth_104(_root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }

    pub fn sorted_array_to_bst_108(_nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}
