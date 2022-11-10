fn main() {
    println!("Hello, world!");
    println!("{}", Solution::num_decodings("11106".into()));
    assert_eq!(2, Solution::num_decodings("12".into()));
    assert_eq!(3, Solution::num_decodings("226".into()));
    assert_eq!(0, Solution::num_decodings("226000".into()));
}

struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        if s[0] == b'0' {
            return 0;
        }

        let mut val = 1;
        let mut prev = 1;
        for i in 0..s.len() - 1 {
            let new = if s[i + 1] != b'0' { 1 } else { 0 } * val
                + prev
                    * match (s[i], s[i + 1]) {
                        (b'1', _) => 1,
                        (b'2', b'0'..=b'6') => 1,
                        _ => 0,
                    };
            prev = val;
            val = new;
        }
        val
    }
}
