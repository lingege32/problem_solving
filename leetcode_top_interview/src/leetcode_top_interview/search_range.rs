struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let upper = Self::find_upper(&nums, target);
        if upper == 0 || nums[upper - 1] != target {
            return vec![-1, -1];
        }

        let lower = Self::find_upper(&nums[..upper], target - 1);
        vec![lower as i32, upper as i32 - 1]
    }

    fn find_upper(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (left + right) / 2;
            if target < nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
