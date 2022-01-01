struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        nums.resize(len + 2, 1);
        nums.rotate_right(1);
        let mut dp = vec![vec![0; len + 2]; len + 2];
        for l in 1..=len {
            for i in 1..=len - l + 1 {
                let j = i + l - 1;
                for k in i..=j {
                    dp[i][j] = dp[i][j]
                        .max(dp[i][k - 1] + nums[i - 1] * nums[k] * nums[j + 1] + dp[k + 1][j]);
                    // println!("l: {}, i: {}, j: {}, k: {}", l, i, j, k);
                    // println!("nums: {:?}", nums);
                    // for d in dp.iter() {
                    //     println!("{:?}", d);
                    // }
                }
            }
        }

        dp[1][len]
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 1, 5, 8];
        assert_eq!(167, Solution::max_coins(nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 5];
        assert_eq!(10, Solution::max_coins(nums));
    }
}
