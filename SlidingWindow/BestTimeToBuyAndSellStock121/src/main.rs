fn main() {
    println!("Hello, world!");
    println!("{}", Solution::max_profit([7,1,5,3,6,4].into()));
    println!("{}", Solution::max_profit([7, 6, 5, 4, 3, 2, 1].into()));
}

struct Solution{}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 { return 0 }

        let mut buy = prices[0];
        let mut profit = 0;

        for sell in prices {
            profit = profit.max(sell - buy);
            buy = buy.min(sell);
        }

        profit
    }
}
