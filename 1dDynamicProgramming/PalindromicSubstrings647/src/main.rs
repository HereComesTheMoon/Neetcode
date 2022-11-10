fn main() {
    println!("Hello, world!");
    println!("{}", Solution::count_substrings("abc".into()));
    println!("{}", Solution::count_substrings("aaa".into()));
    println!("{}", Solution::count_substrings("b".into()));
    println!("{}", Solution::count_substrings("".into()));
    println!("{}", Solution::count_substrings("abcdefg".into()));
    println!("{}", Solution::count_substrings("abbdefg".into()));
    println!("{}", Solution::count_substrings("abbdefgf".into()));
}

struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.len() <= 1 { return s.len() as i32 }
        let s = s.as_bytes();
        let mut dp: Vec<Vec<bool>> = Vec::with_capacity(s.len());
        dp.push(vec![true; s.len()]);
        dp.push(vec![true; s.len()]);
        
        for window in 2..=s.len() {
            let mut row = vec![false; s.len() - window + 1];
            for i in 0..s.len()-window+1 {
                if dp[window-2][i+1] && s[i] == s[i+window-1] {
                    row[i] = true;
                }
            }
            dp.push(row);
        }
        dp[1..]
            .concat()
            .into_iter()
            .filter(|&x| x)
            .count() as i32
    }
}

