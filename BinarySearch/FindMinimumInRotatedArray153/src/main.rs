fn main() {
    println!("Hello, world!");
}

struct Solution;


impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums[0] <= *nums.last().unwrap() {
            return nums[0];
        }
        let first = nums[0];
        let mut i = 0;
        let mut j = nums.len();
        
        while i + 1 < j {
            if nums[(j + i) / 2] < first {
                j = (j + i) / 2;
            } else {
                i = (j + i) / 2;
            }
        }
        nums[(j + i + 1) / 2]
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ex1() {
        assert_eq!(1, Solution::find_min(vec![3,4,5,1,2]));
    }
    
    #[test]
    fn ex2() {
        assert_eq!(0, Solution::find_min(vec![4,5,6,7,0,1,2]));
    }
    
    #[test]
    fn ex3() {
        assert_eq!(11, Solution::find_min(vec![11,13,15,17]));
    }
    
    #[test]
    fn test1() {
        assert_eq!(1, Solution::find_min(vec![1]));
    }
    
    #[test]
    fn test2() {
        for k in 0..20 {
            let mut v: Vec<i32> = (0..20).collect();
            v.rotate_right(k);
            assert_eq!(0, Solution::find_min(v));
        }
        for k in 0..21 {
            let mut v: Vec<i32> = (0..21).collect();
            v.rotate_right(k);
            assert_eq!(0, Solution::find_min(v));
        }
    }
}