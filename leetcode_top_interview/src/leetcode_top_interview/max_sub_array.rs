struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut cur = ans;
        for val in nums.into_iter().skip(1) {
            if cur < 0 {
                cur = val;
            } else {
                cur += val;
            }
            if cur > ans {
                ans = cur;
            }
        }
        ans
    }
}
