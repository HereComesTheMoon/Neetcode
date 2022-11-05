fn main() {
    println!("Hello, world!");
}

struct Solution{}

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        n.count_ones()
    }
}
