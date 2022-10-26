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

// This here doesn't work. Bonus points if you can figure out why.
//impl Solution {
    //pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        //if let Some(root) = &root {
            //std::mem::swap(
                //&mut Solution::invert_tree(
                    //root
                    //.as_ref()
                    //.borrow_mut()
                    //.right
                    //.take()
                    //),
                //&mut Solution::invert_tree(
                    //root
                    //.as_ref()
                    //.borrow_mut()
                    //.left
                    //.take()
                    //)
                //);
        //}
        //root
    //}
//}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = &root {
            let right = root.as_ref().borrow_mut().right.take();
            let left = root.as_ref().borrow_mut().left.take();

            let right = Solution::invert_tree(right);
            let left = Solution::invert_tree(left);

            root.as_ref().borrow_mut().left = right;
            root.as_ref().borrow_mut().right = left;
        }
        root
    }
}
