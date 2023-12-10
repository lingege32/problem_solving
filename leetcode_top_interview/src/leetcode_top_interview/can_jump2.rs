struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut max = 0;
        let mut step = 0;
        for (idx, &jump) in nums[..nums.len()-1].iter().enumerate() {
            max = max.max(idx + jump as usize);
            if cur == idx {
                step += 1;
                cur = max;
            }
        }
        step
    }
}
