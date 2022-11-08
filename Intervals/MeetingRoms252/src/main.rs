fn main() {
    println!("Hello, world!");

    {
        let res = Solution::can_attend_meetings(vec![
                                      vec![0, 30],
                                      vec![5, 10],
                                      vec![15, 20]
        ]);
        assert_eq!(res, false);
    }
}

struct Solution{}

impl Solution {
    fn can_attend_meetings(mut meetings: Vec<Vec<i32>>) -> bool {
        if meetings.len() <= 1 { return true }
        meetings.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        println!("{:?}", meetings);

        let mut last = meetings[0][1];
        for v in meetings {
            if v[0] < last {
                return false
            } else {
                last = last.max(v[1]);
            }
        }
        true
    }
}
