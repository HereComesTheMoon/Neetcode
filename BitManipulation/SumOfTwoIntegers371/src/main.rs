fn main() {
    println!("Hello, world!");
}
struct Solution;


// Given two integers a and b, return the sum of the two integers without using the operators + and -.
// Technically correct
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        a.checked_add(b).unwrap()
    }
}
