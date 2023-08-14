struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        for (idx, val) in nums.into_iter().enumerate() {
            match hm.get(&(target - val)) {
                Some(other_idx) => {
                    return vec![(*other_idx) as i32, idx as i32];
                }
                None => {
                    hm.insert(val, idx);
                }
            }
        }
        return vec![];
    }
}
