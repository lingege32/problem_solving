struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut ret = vec![];
        nums.sort_unstable();

        for (idx, &val) in nums.iter().enumerate() {
            if idx != 0 && val == nums[idx - 1] {
                continue;
            }
            let (mut left, mut right) = (idx + 1, nums.len() - 1);
            while left < right {
                let l = nums[left];
                let r = nums[right];
                let total = l + r + val;
                if total < 0 {
                    left += 1;
                } else if total > 0 {
                    right -= 1;
                } else {
                    ret.push(vec![val, l, r]);
                    left += 1;
                    while left < right && nums[left] == l {
                        left += 1
                    }
                    right -= 1;
                    while left < right && nums[right] == r {
                        right -= 1;
                    }
                }
            }
        }

        ret
    }
}
