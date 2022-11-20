fn main() {
    println!("Hello, world!");
}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type Node = Option<Box<ListNode>>;

use std::collections::VecDeque;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut lists: VecDeque<Node> = lists.into();
        while lists.len() > 1 {
            let a = lists.pop_back().unwrap();
            let b = lists.pop_back().unwrap();
            lists.push_front(merge(a, b));
        }

        lists.pop_back().unwrap()
    }
}

pub fn merge(mut a: Node, mut b: Node) -> Node {
    if a.is_none() {
        return b;
    }
    if b.is_none() {
        return a;
    }
    if a.as_ref().unwrap().val > b.as_ref().unwrap().val {
        std::mem::swap(&mut a, &mut b);
    }

    let mut head = a;
    let mut a = head.as_mut();

    while a.is_some() {
        if b.is_none() {
            break;
        }
        if a.as_ref().unwrap().next.is_none() {
            a.as_mut().unwrap().next = b;
            break;
        }
        if a.as_ref().unwrap().next.as_ref().unwrap().val < b.as_ref().unwrap().val {
            a = a.unwrap().next.as_mut();
        } else {
            std::mem::swap(&mut a.as_mut().unwrap().next, &mut b);
            a = a.unwrap().next.as_mut();
        }
    }

    head
}

pub fn new(v: &[i32]) -> Node {
    if v.is_empty() {
        return None;
    }

    let mut n = ListNode::new(v[0]);
    n.next = new(&v[1..]);
    Some(Box::new(n))
}

pub fn print_list(node: &Node) {
    if node.is_none() {
        println!();
        return;
    }
    print!("{} -> ", node.as_ref().unwrap().val);
    print_list(&node.as_ref().unwrap().next);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let lists = vec![new(&[1, 4, 5]), new(&[1, 3, 4]), new(&[2, 6])];
        assert_eq!(
            Solution::merge_k_lists(lists),
            new(&[1, 1, 2, 3, 4, 4, 5, 6])
        );
    }

    #[test]
    fn ex2() {
        let lists = vec![];
        assert_eq!(Solution::merge_k_lists(lists), new(&[]));
    }

    #[test]
    fn ex3() {
        let lists = vec![new(&[])];
        assert_eq!(Solution::merge_k_lists(lists), new(&[]));
    }
}
