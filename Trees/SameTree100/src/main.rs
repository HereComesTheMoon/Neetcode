fn main() {
    println!("Hello, world!");
}

struct Solution;

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        helper(&p, &q)
    }
}

pub fn helper(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() != q.is_none() {
        return false;
    }
    if p.is_none() {
        return true;
    }
    let p = p.as_ref().unwrap();
    let q = q.as_ref().unwrap();
    if p.borrow().val != q.borrow().val {
        return false;
    }
    return helper(&p.borrow().left, &q.borrow().left)
        && helper(&p.borrow().right, &q.borrow().right);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let p = {
            let l = TreeNode::new(2);
            let r = TreeNode::new(3);
            let mut p = TreeNode::new(1);
            p.left = Some(Rc::new(RefCell::new(l)));
            p.right = Some(Rc::new(RefCell::new(r)));
            Some(Rc::new(RefCell::new(p)))
        };

        let q = {
            let l = TreeNode::new(2);
            let r = TreeNode::new(3);
            let mut p = TreeNode::new(1);
            p.left = Some(Rc::new(RefCell::new(l)));
            p.right = Some(Rc::new(RefCell::new(r)));
            Some(Rc::new(RefCell::new(p)))
        };

        assert!(Solution::is_same_tree(p, q));
    }

    #[test]
    fn ex2() {
        let f = |x: TreeNode| Some(Rc::new(RefCell::new(x)));

        let p = {
            let l = f(TreeNode::new(2));
            let mut p = TreeNode::new(1);
            p.left = l;
            f(p)
        };

        let q = {
            let r = f(TreeNode::new(2));
            let mut p = TreeNode::new(1);
            p.right = r;
            f(p)
        };

        assert!(!Solution::is_same_tree(p, q));
    }

    #[test]
    fn ex3() {
        let f = |x: TreeNode| Some(Rc::new(RefCell::new(x)));

        let p = {
            let l = f(TreeNode::new(2));
            let r = f(TreeNode::new(1));
            let mut p = TreeNode::new(1);
            p.left = l;
            p.right = r;
            f(p)
        };

        let q = {
            let l = f(TreeNode::new(1));
            let r = f(TreeNode::new(2));
            let mut p = TreeNode::new(1);
            p.left = l;
            p.right = r;
            f(p)
        };

        assert!(!Solution::is_same_tree(p, q));
    }
}
