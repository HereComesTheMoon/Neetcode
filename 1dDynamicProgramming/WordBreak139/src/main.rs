use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

// Doesn't work like this. Timeout error! Recursion too expensive on input 
// "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"
// ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<&[u8]> = word_dict.iter().map(|w| w.as_bytes()).collect();
        let s = s.as_bytes();
        
        Self::rec(s, &words)
    }
    
    fn rec(s: &[u8], words: &HashSet<&[u8]>) -> bool {
        if s.is_empty() {
            return true;
        }
        for k in 0..s.len() {
            if words.contains(&s[..=k]) && Self::rec(&s[k+1..], words) {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ex1() {
        assert!(Solution::word_break("leetcode".into(), vec!["leet".into(), "code".into()]));
    }
    
    #[test]
    fn ex2() {
        assert!(Solution::word_break("applepenapple".into(), vec!["apple".into(), "pen".into()]));
    }
    
    #[test]
    fn ex3() {
        assert!(!Solution::word_break("catsandog".into(), vec!["cats".into(), "dog".into(), "sand".into(), "and".into(), "cat".into()]));
    }

    #[test]
    fn test1() {
        assert!(Solution::word_break("aaabbbaaa".into(), vec!["a".into(), "abbba".into()]));
        assert!(!Solution::word_break("aaabbbaaa".into(), vec!["a".into(), "bb".into()]));
        assert!(Solution::word_break("aaabbbaaa".into(), vec!["a".into(), "bb".into(), "aaabbbaaa".into()]));
    }
}