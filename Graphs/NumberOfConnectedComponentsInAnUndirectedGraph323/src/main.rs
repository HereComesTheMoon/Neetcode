fn main() {
    println!("Hello, world!");
    assert_eq!(
        2,
        Solution::count_components(4, vec![vec![0, 1], vec![2, 0]])
    );

    assert_eq!(0, Solution::count_components(0, vec![]));

    assert_eq!(1, Solution::count_components(1, vec![vec![1, 1]]));
}

struct SetForest(Vec<Node>);

#[derive(Clone, Copy)]
struct Node {
    next: usize,
    size: usize,
}

impl SetForest {
    pub fn new(n: usize) -> Self {
        SetForest((0..n).map(|x| Node { next: x, size: 0 }).collect())
    }

    pub fn find(&mut self, mut n: usize) -> usize {
        while self.0[n].next != n {
            self.0[n] = self.0[self.0[n].next];
            n = self.0[n].next;
        }
        n
    }

    pub fn merge(&mut self, a: usize, b: usize) {
        if self.find(a) == self.find(b) {
            return;
        }
        if self.0[a].size < self.0[b].size {
            self.0[a].next = b;
            self.0[b].size += self.0[a].size;
        } else {
            self.0[b].next = a;
            self.0[a].size += self.0[b].size;
        }
    }
}

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut forest = SetForest::new(n as usize);
        for v in edges {
            forest.merge(v[0] as usize, v[1] as usize);
        }

        (0..n as usize)
            .map(|x| forest.find(x))
            .collect::<HashSet<usize>>()
            .len() as i32
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
