fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for r in 0..matrix.len() / 2 {
            for i in 0..matrix.len() - 1 - 2 * r {
                rot(matrix, r, i);
            }
        }
    }
}

#[inline]
pub fn rot(m: &mut Vec<Vec<i32>>, r: usize, i: usize) {
    let l = m.len();
    let n = m[0].len();

    let mut temp = m[r][r + i];
    temp = std::mem::replace(&mut m[r + i][n - 1 - r], temp);
    temp = std::mem::replace(&mut m[l - 1 - r][n - 1 - i - r], temp);
    temp = std::mem::replace(&mut m[l - 1 - r - i][r], temp);
    m[r][r + i] = temp;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut m);

        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn ex2() {
        let mut m = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut m);
        assert_eq!(
            m,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn test1() {
        let mut m = vec![
            vec![1, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 3],
            vec![4, 4, 4, 4, 4],
            vec![5, 5, 5, 5, 5],
        ];
        Solution::rotate(&mut m);
        assert_eq!(
            m,
            vec![
                vec![5, 4, 3, 2, 1],
                vec![5, 4, 3, 2, 1],
                vec![5, 4, 3, 2, 1],
                vec![5, 4, 3, 2, 1],
                vec![5, 4, 3, 2, 1],
            ]
        );
    }

    #[test]
    fn test2() {
        let mut m = vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 3, 3],
            vec![4, 4, 4, 4, 4, 4],
            vec![5, 5, 5, 5, 5, 5],
            vec![6, 6, 6, 6, 6, 6],
        ];
        Solution::rotate(&mut m);
        assert_eq!(
            m,
            vec![
                vec![6, 5, 4, 3, 2, 1],
                vec![6, 5, 4, 3, 2, 1],
                vec![6, 5, 4, 3, 2, 1],
                vec![6, 5, 4, 3, 2, 1],
                vec![6, 5, 4, 3, 2, 1],
                vec![6, 5, 4, 3, 2, 1],
            ]
        );
    }
}
