fn main() {
    println!("Hello, world!");
}
struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        v.sort();

        let mut now = 0;
        while now < v.len() - 1 {
            let mut pos = now + 1;
            while pos < v.len() && v[pos][0] <= v[now][1] {
                v[now][1] = i32::max(v[now][1], v[pos][1]);
                v[pos][0] = 0;
                v[pos][1] = -1;
                pos += 1;
            }
            now = pos;
        }

        v.into_iter().filter(|v| v[0] <= v[1]).collect()
    }
    
    // fn count_overlaps(v: &)

    pub fn merge(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();

        let mut now = 0;
        while now < v.len() - 1 {
            let mut pos = now + 1;
            while pos < v.len() && v[pos][0] <= v[now][1] {
                v[now][1] = i32::max(v[now][1], v[pos][1]);
                v[pos][0] = 0;
                v[pos][1] = -1;
                pos += 1;
            }
            now = pos;
        }

        v.into_iter().filter(|v| v[0] <= v[1]).collect()
    }
}
