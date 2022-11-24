fn main() {
    println!("Hello, world!");
}

struct Solution;


impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut trie = Trie::new();
        for w in word_dict {
            trie.insert(w);
        }
        let trie = trie;
        let s = s.as_bytes();
        let mut valid_start = vec![false; s.len()];
        Self::mark_start(&s, &trie, &mut valid_start);

        for k in 0..s.len() {
            if valid_start[k] {
                Self::mark_start(&s[k + 1..], &trie, &mut valid_start[k + 1..]);
            }
        }

        *valid_start.last().unwrap()
    }

    fn mark_start(s: &[u8], root: &Trie, valid_start: &mut [bool]) {
        let mut node = &root.head;

        for pos in 0..s.len() {
            if let None = node.next[(s[pos] - b'a') as usize] {
                break;
            } else {
                node = node.next[(s[pos] - b'a') as usize].as_ref().unwrap();
                valid_start[pos] |= node.word;
            }
        }
    }
}

struct Trie {
    head: Box<Node>,
}

struct Node {
    next: [Option<Box<Node>>; 26],
    word: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            next: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            word: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            head: Box::new(Node::new()),
        }
    }

    fn insert(&mut self, word: String) {
        let mut n = &mut self.head;
        for &c in word.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                n.next[(c - b'a') as usize] = Some(Box::new(Node::new()));
            }
            n = n.next[(c - b'a') as usize].as_mut().unwrap();
        }
        n.word = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::word_break(
            "leetcode".into(),
            vec!["leet".into(), "code".into()]
        ));
    }

    #[test]
    fn ex2() {
        assert!(Solution::word_break(
            "applepenapple".into(),
            vec!["apple".into(), "pen".into()]
        ));
    }

    #[test]
    fn ex3() {
        assert!(!Solution::word_break(
            "catsandog".into(),
            vec![
                "cats".into(),
                "dog".into(),
                "sand".into(),
                "and".into(),
                "cat".into()
            ]
        ));
    }

    #[test]
    fn test1() {
        assert!(Solution::word_break(
            "aaabbbaaa".into(),
            vec!["a".into(), "abbba".into()]
        ));
        assert!(!Solution::word_break(
            "aaabbbaaa".into(),
            vec!["a".into(), "bb".into()]
        ));
        assert!(Solution::word_break(
            "aaabbbaaa".into(),
            vec!["a".into(), "bb".into(), "aaabbbaaa".into()]
        ));
    }

    #[test]
    fn test2() {
        let mut s = ["a"; 151];
        s[150] = "b";
        let s: String = s.into_iter().collect();
        let dict = [
            "a",
            "aa",
            "aaa",
            "aaaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaa",
            "aaaaaaaaaa",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        assert!(!Solution::word_break(s, dict));
    }
}
