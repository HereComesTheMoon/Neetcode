fn main() {
    println!("Hello, world!");
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

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut last = None;
        loop {
            if head.is_none() {
                return last;
            }
            std::mem::swap(&mut head.as_mut().unwrap().next, &mut last);
            std::mem::swap(&mut last, &mut head);
        }
    }
}
