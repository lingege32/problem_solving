struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let odd_in_n = ((n + 1) / 2) as i64;
        let even_in_n = n as i64 - odd_in_n;
        let odd_in_m = ((m + 1) / 2) as i64;
        let even_in_m = m as i64 - odd_in_m;

        odd_in_n * even_in_m + even_in_n * odd_in_m
    }
}
