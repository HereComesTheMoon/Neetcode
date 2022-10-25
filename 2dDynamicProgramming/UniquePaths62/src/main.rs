fn main() {
    //let n = 35;
    //let fact: u128 = (1..=n).product();
    //println!("{}", fact);

    // Here f32 breaks
    //Solution::unique_paths(19, 13);


    //Solution::unique_paths(16, 16);
}

struct Solution{}


impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        nchoosek(m + n - 2, n - 1)
    }
}

// This is good enough for LeetCode, but I would not trust it for arbitrary values of n and k
// Floating point rounding is concerning. This breaks if you use f32 instead.
fn nchoosek(n: i32, k: i32) -> i32 {
    let k = if 2*k > n {
        n - k
    } else {
        k
    };
    let n = n as f64 + 1.;
    
    (1..=k).map(|i| i as f64).fold(1.0, |fin, i| fin * (n-i) / i).round() as i32
}

// Using ints has its own issues
// (1..=n).product() overflows i32 for n = 12, u64 for n = 21, and u128 for n = 35
// Needs bigint to work properly
fn nchoosek2(n: i32, k: i32) -> i32 {
    let k = if 2*k > n {
        (n - k) as u128
    } else {
        k as u128
    };

    let n = n as u128;

    ((n-k+1..=n).product::<u128>() / (1..=k).product::<u128>()) as i32
}
