struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hs = nums.into_iter().collect::<HashSet<_>>();
        let mut max = 0;
        for &n in &hs {
            if !hs.contains(&(n - 1)) {
                let mut count = 1;
                let mut next = n + 1;
                while hs.contains(&next) {
                    next += 1;
                    count += 1;
                }
                max = max.max(count);
            }
        }
        max
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let ans = 4;
        assert_eq!(Solution::longest_consecutive(nums), ans);
    }
    #[test]
    fn test_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let ans = 9;
        assert_eq!(Solution::longest_consecutive(nums), ans);
    }
}
