fn main() {
    println!("Hello, world!");
}

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let bag: HashSet<i32> = nums.into_iter().collect();
        
        let mut longest = 0;
        for x in bag.iter() {
            if !bag.contains(&(x - 1)) {
                let mut run = 1;
                let mut x = x + 1;
                while bag.contains(&x) {
                    x += 1;
                    run += 1;
                }
                longest = longest.max(run);
            }
        }
        longest
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ex1() {
        assert_eq!(Solution::longest_consecutive(vec![100,4,200,1,3,2]), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }
}
