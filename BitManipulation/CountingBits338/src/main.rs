fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::count_bits(0));
    println!("{:?}", Solution::count_bits(5));
    println!("{:?}", Solution::count_bits(6));
    println!("{:?}", Solution::count_bits(7));
    println!("{:?}", Solution::count_bits(8));
    println!("{:?}", Solution::count_bits(9));
}

struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut v = vec![0_i32; n + 1];
        let mut k = 1;
        let mut boundary = 1;
        while k <= n {
            v[k] = v[k % boundary] + 1;
            k += 1;

            if k == 2*boundary {
                boundary *= 2;
            }
        }

        v
    }
}
