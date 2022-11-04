fn main() {
    println!("Hello, world!");

    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let c = ListNode::new(3);

    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));

    for x in a.iter() {
        println!("{}", x);
    }
}


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution{}

pub struct Iter<'a> {
    next: Option<&'a ListNode>,
}

impl ListNode {
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter { next: Some(self) }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() { return list2 }
        if list2.is_none() { return list1 }
        let mut v: Vec<i32> = list1.unwrap().iter().chain(list2.unwrap().iter()).copied().collect();
        v.sort_unstable();

        let mut last = None;
        for &x in v.iter().rev() {
            let next = Some(Box::new(ListNode{ val: x, next: last.take()}));
            last = next;
        }
        last
    }
}
