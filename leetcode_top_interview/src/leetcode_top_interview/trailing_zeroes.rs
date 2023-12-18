struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn trailing_zeroes(n: i32) -> i32 {
        /*
         * How many 5 decides how many trailing 0.
         * Because 10 is 2*5 and 2 is more than 5 always.
         * Therefore, 5 is dominate the trailing 0 number.
         */
        n / 5 + n / 25 + n / 125 + n / 625 + n / 3125
    }
}
