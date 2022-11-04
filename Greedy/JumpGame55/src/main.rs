fn main() {
    println!("Hello, world!");
}


struct Solution{}


impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut need_to_reach = nums.len() - 1;
        
        let mut i = nums.len() - 2;
        while i != 0 {
            if i + nums[i] as usize >= need_to_reach {
                need_to_reach = i;
            }
            i -= 1;
        }
        need_to_reach <= nums[0] as usize
    }
}
