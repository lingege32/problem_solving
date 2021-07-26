struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            nums[0]
        } else {
            let len = nums.len();
            let mut dp = vec![0; len];
            dp[0] = nums[0];
            for idx in 1..len {
                let max = dp[0..idx-1].iter().max().unwrap_or(&0);
                dp[idx] = *max+nums[idx];
            }

            dp[len-1].max(dp[len-2])
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(4, Solution::rob(nums));
    }
    #[test]
    fn test_2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(12, Solution::rob(nums));
    }
}
