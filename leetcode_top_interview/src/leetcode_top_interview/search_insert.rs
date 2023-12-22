struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (right - left) / 2 + left;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}
