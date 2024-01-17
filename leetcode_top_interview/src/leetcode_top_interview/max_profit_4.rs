struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut buy = vec![i32::MAX; k as usize];
        let mut profit = vec![0; k as usize];

        for price in prices.into_iter() {
            buy[0] = buy[0].min(price);
            profit[0] = profit[0].max(price - buy[0]);
            for idx in 1..k as usize {
                buy[idx] = buy[idx].min(price - profit[idx - 1]);
                profit[idx] = profit[idx].max(price - buy[idx]);
            }
        }

        *profit.last().unwrap()
    }
}
