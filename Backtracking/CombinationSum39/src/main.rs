fn main() {
    println!("Hello, world!");
}

// This can probably still be improved. Try figuring out an iterative solution sometime

struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        rec(&candidates, target)
    }
}

pub fn rec(cand: &[i32], target: i32) -> Vec<Vec<i32>> {
    if target < 0 || cand.is_empty() {
        return vec![];
    }
    if target == 0 {
        return vec![vec![]];
    }
    let a = rec(cand, target - cand[0]);
    let b = rec(&cand[1..], target);

    a.into_iter()
        .map(|mut sol| {
            sol.push(cand[0]);
            sol
        })
        .chain(b.into_iter())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let m = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        println!("{:?}", m);
        assert_eq!(m, vec![vec![3, 2, 2], vec![7]]);
    }

    #[test]
    fn ex2() {
        let m = Solution::combination_sum(vec![2, 3, 5], 8);
        println!("{:?}", m);
        assert_eq!(m, vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]]);
    }

    #[test]
    fn ex3() {
        let m = Solution::combination_sum(vec![2], 1);
        println!("{:?}", m);
        assert_eq!(m, Vec::<Vec::<i32>>::new());
    }
}
