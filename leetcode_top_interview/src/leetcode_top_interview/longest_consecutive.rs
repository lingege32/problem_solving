struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        let mut ans = 0;
        for &n in nums.iter() {
            if !nums.contains(&(n - 1)) {
                let mut dis = 1;
                while nums.contains(&(n + dis)) {
                    dis += 1;
                }
                ans = ans.max(dis);
            }
        }
        ans
    }
}
