fn main() {
    println!("Hello, world!");
    {
        let root = from_vect(&[Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5)].into(),
        0);

        let p = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        Solution::lowest_common_ancestor(root, p, q);
    }
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

struct Solution{}
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

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


impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut a = p.unwrap().as_ref().borrow().val;
        let mut c = q.unwrap().as_ref().borrow().val;
        if c < a {
            std::mem::swap(&mut a, &mut c);
        }

        fn helper(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
            let val = root.as_ref().unwrap().as_ref().borrow().val;
            println!("VALUE: {}", val);
            if p <= val && val <= q {
                return root;
            }
            if p < val {
                let l = root.unwrap().as_ref().borrow().left.clone();
                return helper(l, p, q);
            }
            let r = root.unwrap().as_ref().borrow().right.clone();
            return helper(r, p, q);
        }

        helper(root, a, c)
    }
}
