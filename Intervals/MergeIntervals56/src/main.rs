fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn merge(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();

        let mut now = 0;
        while now < v.len() - 1 {
            let mut pos = now + 1;
            while pos < v.len() && v[pos][0] <= v[now][1] {
                v[now][1] = i32::max(v[now][1], v[pos][1]);
                v[pos][0] = 0;
                v[pos][1] = -1;
                pos += 1;
            }
            now = pos;
        }

        v.into_iter().filter(|v| v[0] <= v[1]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let invs = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let invs = Solution::merge(invs);
        assert_eq!(invs, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn ex2() {
        let invs = vec![vec![1, 4], vec![4, 5]];
        let invs = Solution::merge(invs);
        assert_eq!(invs, vec![vec![1, 5]]);
    }

    #[test]
    fn test1() {
        let invs = vec![vec![1, 1]];
        let invs = Solution::merge(invs);
        assert_eq!(invs, vec![vec![1, 1]]);
    }

    #[test]
    fn test2() {
        let invs = vec![vec![1, 4], vec![1, 4]];
        let invs = Solution::merge(invs);
        assert_eq!(invs, vec![vec![1, 4]]);
    }
}
