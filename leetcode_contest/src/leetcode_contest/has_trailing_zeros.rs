struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        nums.into_iter().filter(|x| x % 2 == 0).take(2).count() == 2
    }
}
