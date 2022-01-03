struct Solution {}
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut ans = map.into_iter().collect::<Vec<_>>();
        ans.sort_by(|x, y| y.1.cmp(&x.1));
        ans.into_iter().take(k as usize).map(|(x, _)| x).collect()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut ans = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        ans.sort_unstable();
        assert_eq!(vec![1, 2], ans);
    }
    #[test]
    fn test_2() {
        let mut ans = Solution::top_k_frequent(vec![1], 1);
        ans.sort_unstable();
        assert_eq!(vec![1], ans);
    }
}
