fn main() {
    println!("Hello, world!");

    Solution::length_of_longest_substring("abba".into());
}

struct Solution{}


use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.bytes();
        let mut window = HashMap::with_capacity(95); // #printable ASCII characters
        let mut i = 0;
        let mut max_width = 0;
        
        for (j, x) in s.into_iter().enumerate() {
            match window.insert(x, j) {
                Some(index) => { i = i.max(index + 1) },
                None => {},
            }
            max_width = max_width.max(1 + j - i);
        }
        
        max_width as i32
    }
}
