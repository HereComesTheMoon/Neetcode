fn main() {
    println!("Hello, world!");
}
struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut v: Vec<Vec<i32>>) -> i32 {
        v.sort();

        for now in 0..v.len() - 1 {
            if v[now][1] <= v[now + 1][0] {
                continue;
            }
            if v[now][1] <= v[now + 1][1] {
                v.swap(now, now + 1);
            }
            v[now][0] = 0;
            v[now][1] = -1;
        }

        v.into_iter().filter(|v| v[0] > v[1]).count() as i32
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ex1() {
        let v = vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]];
        assert_eq!(Solution::erase_overlap_intervals(v), 1);
        
    }
    
    #[test]
    fn ex2() {
        let v = vec![vec![1,2], vec![1, 2], vec![1, 2]];
        assert_eq!(Solution::erase_overlap_intervals(v), 2);
        
    }
    
    #[test]
    fn ex3() {
        let v = vec![vec![1,2], vec![2,3]];
        assert_eq!(Solution::erase_overlap_intervals(v), 0);
        
    }
}
