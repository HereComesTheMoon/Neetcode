fn main() {
    println!("Hello, world!");
}

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

struct Solution;

type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = length(&head);
        if len == n as usize {
            return head.unwrap().next;
        }

        let mut ptr = head.as_mut();
        for _ in 1..len - (n as usize) {
            ptr = ptr.unwrap().next.as_mut();
        }

        let temp = std::mem::take(&mut ptr.as_mut().unwrap().next);
        if let Some(node) = temp {
            ptr.unwrap().next = node.next;
        }
        head
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
        let head = new(&[1, 2, 3, 4, 5]);
        let head = Solution::remove_nth_from_end(head, 2);
        assert_eq!(head, new(&[1, 2, 3, 5]));
    }

    #[test]
    fn ex2() {
        let head = new(&[1]);
        let head = Solution::remove_nth_from_end(head, 1);
        assert_eq!(head, new(&[]));
    }

    #[test]
    fn ex3() {
        let head = new(&[1, 2]);
        let head = Solution::remove_nth_from_end(head, 1);
        assert_eq!(head, new(&[1]));
    }

    #[test]
    fn ex4() {
        let head = new(&[1, 2]);
        let head = Solution::remove_nth_from_end(head, 2);
        assert_eq!(head, new(&[2]));
    }
}
