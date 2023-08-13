struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();

        if n == 1 {
            return false;
        }

        let mut dp = [true, false, nums[0] == nums[1]];

        for i in 2..n {
            let mut current_dp = false;

            if nums[i] == nums[i - 1] && dp[1] {
                current_dp = true;
            } else if nums[i] == nums[i - 1] && nums[i] == nums[i - 2] && dp[0] {
                current_dp = true;
            } else if nums[i] - nums[i - 1] == 1 && nums[i - 1] - nums[i - 2] == 1 && dp[0] {
                current_dp = true;
            }

            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = current_dp;
            if dp[0] == false && dp[1] == false && dp[2] == false {
                break;
            }
        }

        dp[2]
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![4, 4, 4, 5, 6];
        let ans = true;
        assert_eq!(ans, Solution::valid_partition(nums));
    }
    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 2];
        let ans = false;
        assert_eq!(ans, Solution::valid_partition(nums));
    }
    #[test]
    fn test_3() {
        let nums = vec![10, 20, 30];
        let ans = false;
        assert_eq!(ans, Solution::valid_partition(nums));
    }
}
