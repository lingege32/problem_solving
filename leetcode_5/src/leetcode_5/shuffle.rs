struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut nums = nums;
        let another = nums[0..n as usize].to_vec();
        let (mut i1, mut i2) = (0, n as usize);
        for idx in (0..2*n as usize).step_by(2) {
            nums[idx] = another[i1];
            nums[idx+1] = nums[i2];
            i1+=1;
            i2+=1;
        }
        nums
    }
}