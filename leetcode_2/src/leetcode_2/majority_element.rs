struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let threshold = nums.len() / 2 + nums.len() % 2;
        let mut hm = HashMap::new();
        for i in nums {
            let a = hm.entry(i).or_insert(0);
            *a += 1;
            if *a >= threshold {
                return i;
            }
        }
        0
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }
    #[test]
    fn test_2() {
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
