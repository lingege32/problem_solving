struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev1, mut prev2) = (0, 0);
        for val in nums.into_iter() {
            let best = prev1.max(prev2 + val);
            prev2 = prev1;
            prev1 = best;
        }
        prev1.max(prev2)
    }
}
