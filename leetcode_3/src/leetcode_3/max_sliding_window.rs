struct Solution {}


// monotonic queue
use std::collections::VecDeque;
impl Solution {
    #[allow(dead_code)]
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::<i32>::new();
        let mut ans = Vec::new();
        for idx in 0..nums.len() {
            while let Some(back) = queue.back() {
                if *back < nums[idx] {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back(nums[idx]);
            let new_idx = idx as i32 + 1 - k;
            if new_idx >= 0 {
                let new_idx = new_idx as usize;
                let biggest = *queue.front().unwrap();
                ans.push(biggest);
                if biggest == nums[new_idx] {
                    queue.pop_front();
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let ans = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(ans, Solution::max_sliding_window(nums, k));
    }

    #[test]
    fn test_2() {
        let nums = vec![4, -2];
        let k = 2;
        let ans = vec![4];
        assert_eq!(ans, Solution::max_sliding_window(nums, k));
    }
}
