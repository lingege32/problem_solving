struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = nums.len().min(k as usize + 1);
        if k == 1 {
            return false;
        }
        let mut hm = HashMap::<i32, usize>::new();
        for (idx, val) in nums.into_iter().enumerate() {
            let entry = hm.entry(val);
            match entry {
                std::collections::hash_map::Entry::Occupied(mut prev) => {
                    let p = prev.get_mut();
                    if idx - *p < k {
                        return true;
                    } else {
                        *p = idx;
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(idx);
                }
            }
        }
        false
    }
}
