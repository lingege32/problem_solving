struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn longest_subsequence(mut arr: Vec<i32>, difference: i32) -> i32 {
        let mx = 1_000_00;
        let mut dp = vec![0; 2 * mx + 1];

        for a in arr.iter_mut() {
            *a += mx as i32;
        }

        let mut ans = 1;

        for a in arr.into_iter() {
            let prev = a - difference;
            if prev > 0 {
                dp[a as usize] = dp[a as usize].max(dp[prev as usize] + 1).max(1);
                ans = ans.max(dp[a as usize]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![1, 2, 3, 4];
        let difference = 1;
        let ans = 4;
        assert_eq!(ans, Solution::longest_subsequence(arr, difference));
    }
    #[test]
    fn test_2() {
        let arr = vec![1, 3, 5, 7];
        let difference = 1;
        let ans = 1;
        assert_eq!(ans, Solution::longest_subsequence(arr, difference));
    }
    #[test]
    fn test_3() {
        let arr = vec![1, 5, 7, 8, 5, 3, 4, 2, 1];
        let difference = -2;
        let ans = 4;
        assert_eq!(ans, Solution::longest_subsequence(arr, difference));
    }
    #[test]
    fn test_4() {
        let arr = vec![-6, 6, -8, 0, 7, -8, 5, -7, 10, -10];
        let difference = -6;
        let ans = 2;
        assert_eq!(ans, Solution::longest_subsequence(arr, difference));
    }
}
