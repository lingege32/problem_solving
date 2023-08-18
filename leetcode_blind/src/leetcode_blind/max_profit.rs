struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_value = prices[0];
        prices.into_iter().fold(0, |max, price| {
            min_value = min_value.min(price);
            max.max(price - min_value)
        })
    }
}
