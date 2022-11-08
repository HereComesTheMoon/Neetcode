fn main() {
    println!("Hello, world!");
}

struct Trie<'a> {
    head: Node<'a>,
}

struct Node<'a> {
    next: [Option<&'a mut Node<'a>>; 26],
    word: bool,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Node {
            next: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            word: false,
        }
    }
}

impl<'a> Trie<'a> {
    fn new() -> Self {
        Trie { head: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut n = &mut self.head;
        for &c in word.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                let mut m = Node::new();
                n.next[(c - b'a') as usize] = Some(&mut m);
            }
            n = n.next[0].as_mut().unwrap();
            // n = n.next.get_mut(0).unwrap();
            // n = n.next.get_mut(0).as_deref_mut()
            // n = n.next.get_mut((c - b'a') as usize).unwrap();
            // n = n.next[(c - b'a') as usize].as_mut().unwrap();
        }
        n.word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut n = &self.head;
        for &c in word.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                return false;
            }
            n = n.next[(c - b'a') as usize].unwrap();
        }
        return n.word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut n = &self.head;
        for &c in prefix.as_bytes() {
            if let None = n.next[(c - b'a') as usize] {
                return false;
            }
            n = n.next[(c - b'a') as usize].unwrap();
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
        assert_eq!(t.starts_with("".into()), false);
        assert_eq!(t.starts_with("abcd".into()), false);
    }
}
