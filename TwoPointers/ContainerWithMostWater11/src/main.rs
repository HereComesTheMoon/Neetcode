fn main() {
    println!("Hello, world!");
}


struct Solution{}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut a: usize = 0;
        let mut b: usize = height.len() - 1;
        let mut biggest: i32 = 0;
        while a < b {
            biggest = biggest.max((b-a) as i32 * i32::min(height[b], height[a]));
            if height[a] < height[b] {
                a += 1;
            } else {
                b -= 1;
            }
        }
        biggest
    }
}
