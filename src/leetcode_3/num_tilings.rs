struct Solution();
impl Solution {
    #[allow(dead_code)]
    pub fn num_tilings(n: i32) -> i32 {
        // dp[i][k]: i means the i-th col
        // match k {
        //     0 => top and bottom row are occupied,
        //     1 => only top row is occupied,
        //     2 => only bottom row is occupied,
        // }
        let mut dp = vec![[0u64; 3]; n as usize + 1];

        dp[0][0] = 1;
        dp[1][0] = 1;
        let mod_num = 1000_000_007u64;
        for idx in 2..=n as usize {
            dp[idx][0] =
                (dp[idx - 1][0] + dp[idx - 2][0] + dp[idx - 1][1] + dp[idx - 1][2]) % mod_num;
            dp[idx][1] = (dp[idx - 2][0] + dp[idx - 1][2]) % mod_num;
            dp[idx][2] = (dp[idx - 2][0] + dp[idx - 1][1]) % mod_num;
        }
        dp[n as usize][0] as i32
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::num_tilings(3))
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::num_tilings(1))
    }
}
