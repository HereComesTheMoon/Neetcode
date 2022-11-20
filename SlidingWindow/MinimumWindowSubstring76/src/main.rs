fn main() {
    println!("Hello, world!");
    println!(
        "{:?}",
        Solution::min_window("ADOBECODEBANC".into(), "ABC".into())
    );
}

struct Solution;

use std::collections::HashMap;

struct Window<'a> {
    s: &'a [u8],
    pos: (usize, usize),
    target: HashMap<u8, i32>,
    window: HashMap<u8, i32>,
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        if t.len() == 0 {
            return "".to_string();
        }

        let mut target = HashMap::with_capacity(t.len());
        for &x in t.as_bytes() {
            if let Some(val) = target.get(&x) {
                target.insert(x, val + 1);
            } else {
                target.insert(x, 1);
            }
        }

        let mut win = Window {
            s: s.as_bytes(),
            pos: (0, 0),
            window: target.iter().map(|(&k, &_)| (k, 0)).collect(),
            target,
        };

        if !win.find_next_window() {
            return "".to_string();
        }
        win.shrink_window();

        let mut shortest = (win.pos.0, win.pos.1);

        while win.find_next_window() {
            win.shrink_window();
            if win.pos.1 - win.pos.0 < shortest.1 - shortest.0 {
                shortest = (win.pos.0, win.pos.1);
            }
        }

        s[shortest.0..shortest.1].to_string()
    }
}

impl Window<'_> {
    pub fn find_next_window(&mut self) -> bool {
        if self.pos.1 == self.s.len() {
            return false;
        }
        if self.window.contains_key(&self.s[self.pos.1]) {
            self.window.insert(
                self.s[self.pos.1],
                self.window.get(&self.s[self.pos.1]).unwrap() + 1,
            );
        }
        self.pos.1 += 1;
        while self.pos.1 < self.s.len() && !self.is_window() {
            if self.window.contains_key(&self.s[self.pos.1]) {
                self.window.insert(
                    self.s[self.pos.1],
                    self.window.get(&self.s[self.pos.1]).unwrap() + 1,
                );
            }
            self.pos.1 += 1;
        }
        if self.is_window() {
            true
        } else {
            false
        }
    }

    pub fn is_window(&self) -> bool {
        for (k, v) in &self.target {
            if let Some(val) = self.window.get(&k) {
                if val < &v {
                    return false;
                }
            } else {
                unreachable!()
            }
        }
        true
    }

    pub fn shrink_window(&mut self) {
        assert!(self.is_window());
        while self.is_window() {
            if self.window.contains_key(&self.s[self.pos.0]) {
                self.window.insert(
                    self.s[self.pos.0],
                    self.window.get(&self.s[self.pos.0]).unwrap() - 1,
                );
            }
            self.pos.0 += 1;
        }
        self.pos.0 -= 1;
        self.window.insert(
            self.s[self.pos.0],
            self.window.get(&self.s[self.pos.0]).unwrap() + 1,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".into(), "ABC".into()),
            "BANC".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::min_window("a".into(), "a".into()),
            "a".to_string()
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::min_window("a".into(), "aa".into()),
            "".to_string()
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::min_window("a".into(), "b".into()), "".to_string());
        assert_eq!(
            Solution::min_window("ab".into(), "a".into()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("ab".into(), "b".into()),
            "b".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_window(
                "AAABFOQWDUQWDNQKNEWFOIQJFOQNFQONSSSJSSSJDDD".into(),
                "QQ".into()
            ),
            "QNFQ".to_string()
        );
        assert_eq!(
            Solution::min_window(
                "AAABFOQWDUQWDNQKNEWFOIQJFOQNFQONSSSJSSSJDDD".into(),
                "SSSSSSS".into()
            ),
            "".to_string()
        );
    }
}
