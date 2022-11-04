fn main() {
    println!("Hello, world!");
}


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


use std::rc::Rc;
use std::cell::RefCell;
struct Solution{}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        fn helper(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if root.is_none() { return true }

            let root = root.unwrap();

            if root.borrow().val as i64 <= min || max <= root.borrow().val as i64 {
                return false
            }

            return helper(root.borrow().left.clone(), min, max.min(root.borrow().val as i64)) &&
                       helper(root.borrow().right.clone(), min.max(root.borrow().val as i64), max);
        }

        helper(root, i64::MIN, i64::MAX)
    }
}
