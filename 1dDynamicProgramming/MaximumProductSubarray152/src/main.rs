fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums[0];
        }
        let mut pos = 0;
        let mut neg = 0;
        let mut res = i32::MIN;

        for x in nums {
            if x < 0 {
                std::mem::swap(&mut pos, &mut neg);
            }
            pos = i32::max(pos * x, x);
            neg = i32::min(neg * x, x);
            res = res.max(pos);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
    }

    #[test]
    fn ex2() {
        assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
    }

    #[test]
    fn test1() {
        assert_eq!(1, Solution::max_product(vec![-1, -1]));
        assert_eq!(1, Solution::max_product(vec![-1, -1, -1]));
        assert_eq!(1, Solution::max_product(vec![-1, -1, -1, -1]));
        assert_eq!(24, Solution::max_product(vec![1, 2, 3, -1, 4, -1]));
    }
}
