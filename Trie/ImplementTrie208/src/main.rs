fn main() {
    println!("Hello, world!");
}

struct Trie {
    head: Box<Node>,
}

struct Node {
    next: [Option<Box<Node>>; 26],
    word: bool,
}

impl Node {
    fn new() -> Box<Node> {
        Box::new(Node {
            next: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            word: false,
        })
    }
}

impl Trie {
    fn new() -> Self {
        Trie { head: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut n = &mut self.head;
        for &c in word.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                n.next[(c - b'a') as usize] = Some(Node::new());
            }
            n = n.next[(c - b'a') as usize].as_mut().unwrap();
        }
        n.word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut n = &self.head;
        for &c in word.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                return false;
            }
            n = n.next[(c - b'a') as usize].as_ref().unwrap();
        }
        return n.word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut n = &self.head;
        for &c in prefix.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                return false;
            }
            n = n.next[(c - b'a') as usize].as_ref().unwrap();
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut t = Trie::new();
        t.insert("tilting".into());
        assert_eq!(t.search("tilting".into()), true);
        t.insert("horse".into());
        assert_eq!(t.search("horse".into()), true);
        assert_eq!(t.search("hoarse".into()), false);
        t.insert("hoarse".into());
        assert_eq!(t.search("hoarse".into()), true);

        assert_eq!(t.starts_with("ho".into()), true);
        assert_eq!(t.starts_with("tilting".into()), true);
        assert_eq!(t.search("".into()), false);
        assert_eq!(t.starts_with("".into()), true);
        assert_eq!(t.starts_with("abcd".into()), false);
    }
}
