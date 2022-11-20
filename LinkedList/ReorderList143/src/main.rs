fn main() {
    println!("Hello, world!");
}

struct Solution;

// This kind of task is quite a pain. I looked at another Rust solution to see what sort of approach is even reasonable.

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

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let len = length(head);
        if len <= 2 {
            return;
        }

        let mut ptr: Option<&mut Box<ListNode>> = head.as_mut();
        for _ in 0..(len - 1) / 2 {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        let half = std::mem::take(&mut ptr.unwrap().next);
        let half = rev(half);

        merge(head, half);
    }
}

pub fn length(mut head: &Node) -> usize {
    let mut count = 0;
    while let Some(node) = head {
        count += 1;
        head = &node.next;
    }
    count
}

pub fn rev(mut head: Node) -> Node {
    let mut last = None;
    loop {
        if head.is_none() {
            return last;
        }
        std::mem::swap(&mut head.as_mut().unwrap().next, &mut last);
        std::mem::swap(&mut head, &mut last);
    }
}

pub fn merge(mut left: &mut Node, mut right: Node) {
    loop {
        if left.is_none() {
            return;
        }
        std::mem::swap(&mut left.as_mut().unwrap().next, &mut right);
        left = &mut left.as_mut().unwrap().next;
    }
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
        let mut head = new(&[1, 2, 3, 4]);
        print_list(&head);
        Solution::reorder_list(&mut head);
        print_list(&head);
        assert_eq!(head, new(&[1, 4, 2, 3]));
    }

    #[test]
    fn ex2() {
        let mut head = new(&[1, 2, 3, 4, 5]);
        print_list(&head);
        Solution::reorder_list(&mut head);
        print_list(&head);
        assert_eq!(head, new(&[1, 5, 2, 4, 3]));
    }
}
