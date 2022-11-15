use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::pacific_atlantic(vec![vec![1]]));
    println!(
        "{:?}",
        Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ])
    );
    println!(
        "{:?}",
        Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2]])
    );
}

struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results_p = HashSet::new();
        let mut seen = vec![vec![false; heights[0].len()]; heights.len()];

        for x in 0..heights[0].len() {
            flow(x, 0, &heights, &mut seen, &mut results_p);
        }
        for y in 1..heights.len() {
            flow(0, y, &heights, &mut seen, &mut results_p);
        }

        let mut results_a = HashSet::new();

        for row in &mut seen {
            row.clear();
            row.resize(heights[0].len(), false);
        }
        for x in 0..heights[0].len() {
            flow(x, heights.len() - 1, &heights, &mut seen, &mut results_a);
        }
        for y in 0..heights.len() {
            flow(heights[0].len() - 1, y, &heights, &mut seen, &mut results_a);
        }

        let mut results = Vec::with_capacity(results_p.len());
        for (x, y) in results_p.drain() {
            if results_a.contains(&(x, y)) {
                results.push(vec![x, y]);
            }
        }
        results
    }
}

fn flow(
    x: usize,
    y: usize,
    heights: &Vec<Vec<i32>>,
    seen: &mut Vec<Vec<bool>>,
    results: &mut HashSet<(i32, i32)>,
) {
    results.insert((y as i32, x as i32));
    seen[y][x] = true;

    for (xx, yy) in [
        (x + 1, y),
        (x.overflowing_sub(1).0, y),
        (x, y + 1),
        (x, y.overflowing_sub(1).0),
    ] {
        let known_check = seen.get(yy).and_then(|row| row.get(xx));
        if known_check.is_none() {
            continue;
        }
        if *known_check.unwrap() {
            continue;
        }
        if heights[y][x] <= heights[yy][xx] {
            flow(xx, yy, heights, seen, results);
        }
    }
}
