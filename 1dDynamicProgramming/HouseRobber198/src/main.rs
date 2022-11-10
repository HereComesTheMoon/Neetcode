fn main() {
    println!("Hello, world!");

    assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    assert_eq!(99, Solution::rob(vec![99]));
    assert_eq!(9, Solution::rob(vec![1, 9]));
    assert_eq!(9, Solution::rob(vec![9, 1]));
}

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |acc, x| (acc.1, i32::max(acc.1, acc.0 + x)))
            .1
    }
}
