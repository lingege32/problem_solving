struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut cache = HashSet::<i32>::new();
        for n in nums {
            if !cache.insert(n) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1];
        let ans = true;
        assert_eq!(Solution::contains_duplicate(nums), ans);
    }
    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4];
        let ans = false;
        assert_eq!(Solution::contains_duplicate(nums), ans);
    }
}
