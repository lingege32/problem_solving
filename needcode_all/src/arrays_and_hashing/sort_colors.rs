struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut n = [0;3];
        for &num in nums.iter() {
            n[num as usize]+=1;
        }
        let mut idx = 0;
        for (color, count) in n.into_iter().enumerate() {
            let next = idx+count;
            nums[idx..next].fill(color as i32);
            idx = next;
        }
    }
}