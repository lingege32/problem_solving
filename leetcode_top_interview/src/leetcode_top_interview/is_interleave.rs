struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();
        let s3 = s3.chars().collect::<Vec<_>>();
        Self::inner(&s1, &s2, &s3)
    }
    fn inner(s1: &[char], s2: &[char], s3: &[char]) -> bool {
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;
        for idx in 1..=s2.len() {
            dp[0][idx] = s2[idx - 1] == s3[idx - 1];
            if !dp[0][idx] {
                break;
            }
        }

        for idx in 1..=s1.len() {
            dp[idx][0] = s1[idx - 1] == s3[idx - 1];
            if !dp[idx][0] {
                break;
            }
        }

        for row in 1..=s1.len() {
            for col in 1..=s2.len() {
                let c3 = s3[row + col - 1];
                let choose_1 = dp[row - 1][col] && s1[row - 1] == c3;
                let choose_2 = dp[row][col - 1] && s2[col - 1] == c3;
                dp[row][col] = choose_1 || choose_2;
            }
        }

        *dp.last().unwrap().last().unwrap()
    }
}
