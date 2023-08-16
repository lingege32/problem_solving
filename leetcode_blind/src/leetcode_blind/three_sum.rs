struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut ret = vec![];
        nums.sort_unstable();

        for (idx, val) in nums.iter().enumerate() {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }
            let (mut l, mut r) = (idx + 1, nums.len() - 1);
            while l < r {
                let total = val + nums[l] + nums[r];
                if total == 0 {
                    ret.push(vec![*val, nums[l], nums[r]]);
                    l += 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                } else if total < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        ret
    }
}
