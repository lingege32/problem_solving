struct Solution;
const INVALID: i32 = i32::MAX;
impl Solution {
    #[allow(dead_code)]
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let mut dp = vec![INVALID; amount as usize + 1];
        dp[0] = 0;
        for total in 1..=amount as usize {
            let mut tmp = INVALID;
            for &coin in coins.iter() {
                if coin as usize <= total {
                    let prev = total - coin as usize;
                    if dp[prev] != INVALID {
                        tmp = tmp.min(dp[prev] + 1);
                    }
                }
            }
            dp[total] = tmp;
        }
        let ans = *dp.last().unwrap();
        if ans == INVALID {
            -1
        } else {
            ans
        }
    }
}
