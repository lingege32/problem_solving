struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm = HashMap::<i32, usize>::new();
        for n in nums {
            *hm.entry(n).or_insert(0) += 1;
        }
        let mut v = hm.into_iter().collect::<Vec<_>>();
        let len = v.len();
        v.select_nth_unstable_by_key(len - k as usize, |(_, count)| *count);
        v.into_iter()
            .rev()
            .take(k as usize)
            .map(|(val, _)| val)
            .collect()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let ans = vec![1, 2];
        let mut give = Solution::top_k_frequent(nums, k);
        give.sort();
        assert_eq!(ans, give);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let k = 1;
        let ans = vec![1];
        let mut give = Solution::top_k_frequent(nums, k);
        give.sort();
        assert_eq!(ans, give);
    }
}
