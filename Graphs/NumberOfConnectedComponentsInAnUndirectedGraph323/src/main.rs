fn main() {
    println!("Hello, world!");
}

#[derive(Clone, PartialEq)]
struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    size: u32,
}

impl<'a, 'b> Node<'a> {
    pub fn find(&self) -> &Node {
        let mut node = self;
        // TODO: Path-splitting, update parents. Difficult in Rust, mutability issues
        while node.parent.is_some() {
            node = node.parent.unwrap();
        }
        node
    }

    pub fn merge(&'b mut self, other: &'a mut Node<'b>) {
        if *self.find() == *other.find() {
            return;
        }
        if self.size <= other.size {
            self.size += other.size;
            other.parent = Some(self);
        } else {
            other.size += self.size;
            self.parent = Some(other);
        }
    }
}

struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut nodes = vec![
            Node {
                parent: None,
                size: 0
            };
            n as usize
        ];
        for v in edges {
            let (a, b) = if v[0] <= v[1] {
                (v[0] as usize, v[1] as usize)
            } else {
                (v[1] as usize, v[0] as usize)
            };
            let (head, tail) = nodes.split_at_mut(b);

            Node::merge(&mut head[a], &mut tail[0]);
        }
        unimplemented!()
    }
}

// Given n nodes labeled from 0 to n - 1 and a list of undirected edges (each edge is a pair of nodes), write a function to find the number of connected components in an undirected graph.

// Example 1:
//      0          3
//      |          |
//      1 --- 2    4
// Given n = 5 and edges = [[0, 1], [1, 2], [3, 4]], return 2.

// Example 2:
//      0           4
//      |           |
//      1 --- 2 --- 3
// Given n = 5 and edges = [[0, 1], [1, 2], [2, 3], [3, 4]], return 1.

// Note:
// You can assume that no duplicate edges will appear in edges. Since all edges are undirected, [0, 1] is the same as [1, 0] and thus will not appear together in edges.
