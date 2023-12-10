struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .map(|x| {
                let r = x[1];
                let l = x[0];
                0.max(r - l)
            })
            .sum()
    }
}
