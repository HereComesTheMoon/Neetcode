use std::collections::HashSet;

fn main() {
}

struct Solution{}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());
        
        !nums.iter().all(|x| seen.insert(x))
    }
}
