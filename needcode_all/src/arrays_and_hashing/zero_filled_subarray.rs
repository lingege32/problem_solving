struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut left = 0;
        let mut ret = 0;
        while left < nums.len() {
            if nums[left] == 0 {
                let mut right = left + 1;
                while right < nums.len() && nums[right] == 0 {
                    right += 1;
                }

                let zero_len = (right - left) as i64;
                ret += (zero_len + 1) * zero_len / 2;

                left = right;
            } else {
                left += 1;
            }
        }
        ret
    }
}
