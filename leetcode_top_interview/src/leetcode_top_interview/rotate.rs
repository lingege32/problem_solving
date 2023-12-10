struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let r = k as usize % nums.len();
        nums.rotate_right(r);
    }
}