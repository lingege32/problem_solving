struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_value = prices[0];
        prices.into_iter().fold(0, |max, price| {
            min_value = min_value.min(price);
            max.max(price - min_value)
        })
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }
    #[test]
    fn test_2() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
