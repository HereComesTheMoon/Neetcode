fn main() {
    println!("Hello, world!");
}


struct Solution{}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut now = 0;
        for &x in &nums {
            now = i32::max(x, now + x);
            max = i32::max(max, now);
        }
        max
    }
}
