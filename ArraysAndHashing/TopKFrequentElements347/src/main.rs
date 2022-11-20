use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for &x in nums.iter() {
            if let Some(val) = count.get_mut(&x) {
                *val += 1;
            } else {
                count.insert(x, 1);
            }
        }

        let mut nums: Vec<(usize, i32)> = count.into_iter().map(|(k, v)| (v, k)).collect();
        nums.sort_unstable();

        nums[nums.len() - k as usize..]
            .into_iter()
            .map(|&(_, v)| v)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let sol = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(sol, vec![2, 1]);
    }

    #[test]
    fn ex2() {
        let sol = Solution::top_k_frequent(vec![1], 1);
        assert_eq!(sol, vec![1]);
    }
}
