use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
struct Solution{}

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

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![]
        }
        let mut stack = VecDeque::new();
        let mut output: Vec<Vec<i32>> = vec![vec![]];

        stack.push_back((root.unwrap(), 0));

        let mut last_level = 0;

        while !stack.is_empty() {
            let (next, level) = stack.pop_front().unwrap();
            if level != last_level {
                output.push(vec![next.as_ref().borrow().val]);
                last_level = level;
            } else {
                output.last_mut().unwrap().push(next.as_ref().borrow().val);
            }

            if let Some(l) = next.borrow().left.clone() {
                stack.push_back((l, level + 1));
            };
            if let Some(r) = next.borrow().right.clone() {
                stack.push_back((r, level + 1));
            };
        }
        
        output
    }
}
