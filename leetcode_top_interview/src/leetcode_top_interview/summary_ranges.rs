struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut left = 0;
        let mut ret = vec![];
        for (idx, &val) in nums.iter().enumerate().skip(1) {
            let prev = nums[idx - 1];
            if val > prev + 1 {
                if idx - 1 == left {
                    ret.push(format!("{}", nums[left]));
                } else {
                    ret.push(format!("{}->{}", nums[left], nums[idx - 1]));
                }
                left = idx;
            }
        }
        if nums.len() - 1 == left {
            ret.push(format!("{}", nums[left]));
        } else {
            ret.push(format!("{}->{}", nums[left], nums[nums.len() - 1]));
        }

        ret
    }
}
