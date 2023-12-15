struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0usize, 0usize);
        let len = nums.len();
        let mut total = 0;
        let mut ans = i32::MAX;
        while right < len {
            while right < len && total < target {
                total += nums[right];
                right += 1;
            }
            while left < right && total >= target {
                total -= nums[left];
                ans = ans.min((right - left) as i32);
                left += 1;
            }
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(2, Solution::min_sub_array_len(7, nums));
    }
}
