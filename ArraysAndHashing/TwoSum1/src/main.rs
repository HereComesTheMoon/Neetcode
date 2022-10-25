fn main() {
    let a = Solution::two_sum(vec![2, 7, 11, 15], 9);

    println!("{:?}", a)
}

struct Solution{}


use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        
        for (i, x) in nums.iter().enumerate() {
            match seen.get(&x) {
                Some(&j) => { return vec![i as i32, j as i32]; }
                None => { seen.insert(target - x, i); },
            }
            println!("{:?}", seen);
        }
        unreachable!()
    }
}
