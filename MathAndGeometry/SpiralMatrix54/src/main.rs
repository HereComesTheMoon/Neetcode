fn main() {
    println!("Hello, world!");
}

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sol: Vec<i32> = Vec::with_capacity(matrix.len() * matrix[0].len());
        let mut matrix: VecDeque<VecDeque<i32>> =
            matrix.into_iter().map(|row| row.into()).collect();

        while !matrix.is_empty() && matrix[0].len() != 0 {
            sol.extend(matrix.pop_front().unwrap());
            for row in &mut matrix {
                sol.push(row.pop_back().unwrap());
            }
            if matrix.is_empty() || matrix[0].len() == 0 {
                break;
            }
            sol.extend(matrix.pop_back().unwrap().into_iter().rev());
            for row in &mut matrix.iter_mut().rev() {
                sol.extend(row.drain(0..=0));
            }
        }

        sol
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn ex2() {
        let m = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }

    #[test]
    fn test1() {
        let m = vec![vec![1,2,3,4,5],vec![10,9,8,7,6]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![1,2,3,4,5,6,7,8,9,10]);
    }

    #[test]
    fn test2() {
        let m = vec![vec![7], vec![9], vec![6]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![7,9,6]);
    }

    #[test]
    fn test3() {
        let m = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![1,2,3,4,8,12,16,15,14,13,9,5,6,7,11,10]);
    }
    
    #[test]
    fn test4() {
        let m = vec![vec![2,3,4],vec![5,6,7],vec![8,9,10],vec![11,12,13],vec![14,15,16]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![2,3,4,7,10,13,16,15,14,11,8,5,6,9,12]);
    }    
    
    #[test]
    fn test5() {
        let m = vec![vec![1,2], vec![3,4], vec![5,6], vec![7,8]];
        let sol = Solution::spiral_order(m);
        assert_eq!(sol, vec![1,2,4,6,8,7,5,3]);
    }    
    
}
