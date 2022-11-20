fn main() {
    println!("Hello, world!");
}

// Definitely look at this one again later.
// Need more familiarity with Rust: clone, borrow, refCell handling
// Also, with in-order traversal. Need more practice

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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(mut node_opt: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = Vec::new();

        while node_opt.is_some() || !stack.is_empty() {
            while let Some(node) = node_opt {
                stack.push(node.clone());
                node_opt = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }

                node_opt = node.borrow().right.clone();
            }
        }

        unreachable!()
    }
}
