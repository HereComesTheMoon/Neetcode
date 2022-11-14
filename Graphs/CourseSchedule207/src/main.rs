fn main() {
    println!("Hello, world!");
}

struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut perm = HashSet::with_capacity(num_courses as usize);
        let mut temp = HashSet::with_capacity(num_courses as usize);
        let mut neighbours: HashMap<i32, HashSet<i32>> =
            (0..num_courses).map(|x| (x, HashSet::new())).collect();
        for edge in prerequisites {
            neighbours.get_mut(&edge[0]).unwrap().insert(edge[1]);
        }
        let neighbours = neighbours;

        for x in 0..num_courses {
            if perm.contains(&x) {
                continue;
            }
            if has_cycle(x, &mut perm, &mut temp, &neighbours) {
                return false;
            }
        }
        true
    }
}

fn has_cycle(
    node: i32,
    perm: &mut HashSet<i32>,
    temp: &mut HashSet<i32>,
    neighbours: &HashMap<i32, HashSet<i32>>,
) -> bool {
    if perm.contains(&node) {
        return false;
    }
    if temp.contains(&node) {
        return true;
    }
    temp.insert(node);

    for &next in neighbours.get(&node).unwrap() {
        if has_cycle(next, perm, temp, &neighbours) {
            return true;
        }
    }
    temp.remove(&node);
    perm.insert(node);
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn ex3() {
        assert!(Solution::can_finish(3, vec![vec![1, 0], vec![2, 1]]));
        assert!(Solution::can_finish(3, vec![vec![1, 0], vec![2, 0]]));
        assert!(!Solution::can_finish(1, vec![vec![0, 0]]));
    }
}
