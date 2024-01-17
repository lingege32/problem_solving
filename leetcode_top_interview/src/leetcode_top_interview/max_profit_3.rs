struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy1, mut buy2) = (i32::MAX, i32::MAX);
        let (mut profit, mut total_profit) = (0, 0);

        for price in prices {
            buy1 = buy1.min(price);
            profit = profit.max(price - buy1);

            buy2 = buy2.min(price - profit);
            total_profit = total_profit.max(price - buy2);
        }

        total_profit
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let prices = vec![6,1,4,1,3,1,7];
        assert_eq!(9, Solution::max_profit(prices));
    }
    #[test]
    fn test_2() {
        let prices = vec![1,10,2,11,1,10];
        assert_eq!(19, Solution::max_profit(prices));
    }
}