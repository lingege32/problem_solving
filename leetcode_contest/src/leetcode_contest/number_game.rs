struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        (0..nums.len())
            .step_by(2)
            .map(|idx| vec![nums[idx + 1], nums[idx]])
            .flatten()
            .collect::<Vec<_>>()
    }
}
