struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut dp = vec![vec![0; n as usize + 1]; 2];
        dp[0][0] = 1;

        for i in 1..=goal {
            let cur = (i % 2) as usize;
            let prev = cur ^ 1;
            dp[cur][0] = 0;
            for j in 1..=std::cmp::min(i, n) {
                dp[cur][j as usize] = dp[prev][(j - 1) as usize] * (n - (j - 1)) as i64 % MOD;
                if j > k {
                    dp[cur][j as usize] =
                        (dp[cur][j as usize] + dp[prev][j as usize] * (j - k) as i64) % MOD;
                }
            }
        }

        dp[goal as usize % 2][n as usize] as i32
    }
}
