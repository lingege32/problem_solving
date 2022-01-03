struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hm = HashSet::new();
        for i in nums.iter() {
            hm.insert(i);
        }
        let mut biggest = 0;
        for i in nums.iter() {
            if hm.contains(i) {
                let down_count = (1..)
                    .take_while(|down| -> bool { hm.remove(&(*i - *down)) })
                    .count() as i32;
                let up_count = (1..)
                    .take_while(|down| -> bool { hm.remove(&(*i + *down)) })
                    .count() as i32;
                biggest = biggest.max(1 + up_count + down_count);
            }
        }
        biggest
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [100, 4, 200, 1, 3, 2].to_vec();
        assert_eq!(4, Solution::longest_consecutive(nums));
    }
    #[test]
    fn test_2() {
        let nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1].to_vec();
        assert_eq!(9, Solution::longest_consecutive(nums));
    }
}
