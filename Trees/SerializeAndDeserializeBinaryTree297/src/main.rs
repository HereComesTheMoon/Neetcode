use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}


pub fn from_vect(v: &VecDeque<Option<i32>>, pos: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let val = v.get(pos);
    let root = if let Some(Some(val)) = val {
        Some(Rc::new(RefCell::new( TreeNode {
            val: *val,
            left: from_vect(v, 2*pos+1),
            right: from_vect(v, 2*(pos+1)),
        })))
    } else { None };

    root
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

struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
