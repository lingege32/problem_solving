
struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        for (idx, val) in nums.into_iter().enumerate() {
            match hm.get(&(target - val)) {
                Some(other_idx) => {
                    return vec![(*other_idx) as i32, idx as i32];
                },
                None => {
                    hm.insert(val,idx);
                }
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
