fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut sol = vec![1; nums.len()];

        let mut acc = 1;
        for (k, x) in nums.iter().enumerate() {
            sol[k] *= acc;
            acc *= *x;
        }
        acc = 1;
        for (k, x) in nums.iter().enumerate().rev() {
            sol[k] *= acc;
            acc *= *x;
        }
        sol
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
