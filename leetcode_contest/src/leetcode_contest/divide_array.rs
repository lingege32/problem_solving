struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = vec![];

        for idx in (0..nums.len()).step_by(3) {
            let slice = &nums[idx..idx + 3];
            if slice[2] - slice[0] > k {
                return vec![];
            }
            ret.push(slice.to_owned());
        }

        ret
    }
}
