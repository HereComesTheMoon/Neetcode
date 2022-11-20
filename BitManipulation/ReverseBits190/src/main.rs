fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        (0..32)
            .fold((0, x), |acc, _| (acc.0 << 1 | acc.1 & 1, acc.1 >> 1))
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            964176192
        );
    }
}
