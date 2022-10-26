fn main() {
    println!("Hello, world!");

    // Mind the conventions for fibonacci numbers. Two different ones:
    // F_0 = 0, F_1 = 1, F_2 = 1, ...
    // F_0 = 1, F_1 = 1, F_2 = 2, ...
    for n in 0..=20 {
        println!("n = {:2}. F_n = {}", n, Solution::climb_stairs(n));
    }

    let n = 40;
    println!("n = {:2}. F_n = {}", n, Solution::climb_stairs(n));
}

struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut mem: HashMap<i32, i32> = HashMap::new();
        mem.insert(0, 0);
        mem.insert(1, 1);
        mem.insert(2, 1);

        let res = Solution::helper(&mut mem, n+1);
        res
    }

    fn helper(mut mem: &mut HashMap<i32, i32>, n: i32) -> i32 {
        match mem.get(&n) {
            Some(&val) => val,
            None if n % 2 == 0 => {
                let val_a = Solution::helper(&mut mem, n / 2);
                let val_b = Solution::helper(&mut mem, (n / 2) + 1);
                let res = (2*val_b - val_a)*val_a;
                mem.insert(n, res);
                res
            }
            None if n % 2 == 1 => {
                let val_a = Solution::helper(&mut mem, (n + 1) / 2);
                let val_b = Solution::helper(&mut mem, (n - 1) / 2);
                let res = val_a*val_a + val_b*val_b;
                mem.insert(n, res);
                res
            }
            None => unreachable!(),
        }
    }
}
