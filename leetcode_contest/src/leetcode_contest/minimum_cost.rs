struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_cost(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let l1 = nums[nums.len() / 2] as i64;
        let mut ans = i64::MAX;
        if Self::is_palindromic(l1) {
            ans = ans.min(Self::calculate(l1, &nums));
        }
        for t in (0..l1).rev() {
            if Self::is_palindromic(t) {
                ans = ans.min(Self::calculate(t, &nums));
                break;
            }
        }

        for t in l1 + 1.. {
            if Self::is_palindromic(t) {
                ans = ans.min(Self::calculate(t, &nums));
                break;
            }
        }
        ans
    }
    fn is_palindromic(target: i64) -> bool {
        let s = target.to_string().into_bytes();
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
    fn calculate(target: i64, nums: &[i32]) -> i64 {
        nums.iter().map(|&x| (x as i64 - target).abs()).sum::<i64>()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let v = vec![10, 12, 13, 14, 15];
        let ans = 11;
        assert_eq!(ans, Solution::minimum_cost(v));
    }
    #[test]
    fn test_2() {
        let v = vec![98, 100];
        let ans = 2;
        assert_eq!(ans, Solution::minimum_cost(v));
    }
    #[test]
    fn test_3() {
        let v = vec![1, 2, 3, 4, 5];
        let ans = 6;
        assert_eq!(ans, Solution::minimum_cost(v));
    }
    #[test]
    fn test_5() {
        let v = vec![5, 2, 1];
        let ans = 4;
        assert_eq!(ans, Solution::minimum_cost(v));
    }
    #[test]
    fn test_4() {
        let v = vec![101, 102, 105, 108, 124];
        let ans = 35;
        assert_eq!(ans, Solution::minimum_cost(v));
    }
    #[test]
    fn test_6() {
        let v = vec![150, 722, 102, 628, 272, 539, 753, 161, 814, 930];
        let ans = 2623;
        assert_eq!(ans, Solution::minimum_cost(v));
    }
}
