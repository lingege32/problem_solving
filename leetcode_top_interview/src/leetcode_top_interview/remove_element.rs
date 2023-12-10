struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0usize;
        for i in 0..nums.len() {
            let v = unsafe { *nums.get_unchecked(i) };
            if v != val {
                unsafe {
                    *nums.get_unchecked_mut(idx) = v;
                }
                idx += 1;
            }
        }

        idx as i32
    }
}
