struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let low = low | 1;
        let total = high - low + 1;
        total / 2 + (high % 2)
    }
}