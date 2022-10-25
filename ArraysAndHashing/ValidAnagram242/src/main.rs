fn main() {
    println!("Hello, world!");

    println!("{}", Solution::is_anagram("car".into(), "rat".into()));
}

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count: [i32; 26] = [0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        for c in t.chars() {
            count[(c as u8 - b'a') as usize] -= 1;
            if count[(c as u8 - b'a') as usize] < 0 {
                return false;
            }
        }

        true
    }
}
