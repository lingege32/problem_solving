struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0usize;
        for i in 0..nums.len() {
            let v = unsafe { *nums.get_unchecked(i) };
            let r = unsafe { *nums.get_unchecked_mut(idx) };

            if r != v {
                idx += 1;
                unsafe {
                    *nums.get_unchecked_mut(idx) = v;
                }
            }
        }
        (idx + 1) as i32
    }
}
// impl Solution2 {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         nums.dedup();
//         nums.len() as i32
//     }
// }
