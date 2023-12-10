struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // Work in backward direction. We only need to remember the most recent
        // index that can reach the end of the array.

        if nums.is_empty() {
            return false;
        }

        let mut prev_idx = nums.len() - 1;

        for i in (0..nums.len() - 1).rev() {
            if nums[i] as usize + i >= prev_idx {
                prev_idx = i;
            }
        }

        prev_idx == 0usize
    }
}
