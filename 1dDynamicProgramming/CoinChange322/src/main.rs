fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut coins: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
        coins.sort_unstable();
        let coins = coins;
        let mut mins: Vec<usize> = vec![i32::MAX as usize; 1 + amount as usize];
        mins[0] = 0;
        for x in 1..mins.len() {
            for &y in coins.iter() {
                if x < y {
                    break;
                }
                mins[x] = usize::min(mins[x], 1 + mins[x - y]);
            }
        }
        if i32::MAX as usize <= mins[amount as usize] {
            -1
        } else {
            mins[amount as usize] as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
