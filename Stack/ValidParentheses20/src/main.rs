fn main() {
    println!("Hello, world!");

    assert_eq!(true, Solution::is_valid("()".into()));
    assert_eq!(true, Solution::is_valid("()[]{}".into()));
    assert_eq!(false, Solution::is_valid("(]".into()));
    assert_eq!(false, Solution::is_valid("][".into()));
    assert_eq!(true, Solution::is_valid("(((({[][][()]}))))".into()));
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let s = s.as_bytes();
        let mut stack = Vec::with_capacity((s.len() + 1) / 2);
        for c in s {
            match *c {
                b'(' => {
                    stack.push(b'(');
                }
                b'[' => {
                    stack.push(b'[');
                }
                b'{' => {
                    stack.push(b'{');
                }
                b')' => {
                    if let Some(b'(') = stack.pop() {
                    } else {
                        return false;
                    };
                }
                b']' => {
                    if let Some(b'[') = stack.pop() {
                    } else {
                        return false;
                    };
                }
                b'}' => {
                    if let Some(b'{') = stack.pop() {
                    } else {
                        return false;
                    };
                }
                _ => panic!(),
            }
        }
        stack.is_empty()
    }
}
