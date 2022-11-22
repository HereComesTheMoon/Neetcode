fn main() {
    println!("Hello, world!");
}

struct Solution;


impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let board: Vec<Vec<u8>> = board.into_iter().map(|row| row.iter().map(|c| *c as u8).collect::<Vec<u8>>()).collect();
        let mut visited = (0..board.len()).map(|_| vec![false; board[0].len()]).collect();
        let word = word.as_bytes();
        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if rec(&board, &mut visited, word, (x as isize, y as isize)) {
                    return true
                }
            }
        }
        false
    }
}


pub fn rec(board: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, word: &[u8], pos: (isize, isize)) -> bool {
    let test = visited.get(pos.1 as usize).and_then(|row| row.get(pos.0 as usize));
    if test.is_none() || *test.unwrap() { return false; }
    
    if board[pos.1 as usize][pos.0 as usize] != word[0] { return false; }
    
    if word.len() == 1 { return true; }
    
    visited[pos.1 as usize][pos.0 as usize] = true;
    if rec(board, visited, &word[1..], (pos.0 - 1, pos.1)) {
        return true;
    }
    if rec(board, visited, &word[1..], (pos.0 + 1, pos.1)) {
        return true;
    }
    if rec(board, visited, &word[1..], (pos.0, pos.1 - 1)) {
        return true;
    }
    if rec(board, visited, &word[1..], (pos.0, pos.1 + 1)) {
        return true;
    }
    visited[pos.1 as usize][pos.0 as usize] = false;
    return false;
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ex1() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        assert!(Solution::exist(board, "ABCCED".into()));
    }
    
    #[test]
    fn ex2() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        assert!(Solution::exist(board, "SEE".into()));
    }
    
    #[test]
    fn ex3() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        assert!(!Solution::exist(board, "ABCB".into()));
    }
}