struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ret = vec![1; len];
        let mut prev = 1;
        for idx in 1..len {
            unsafe {
                prev *= nums.get_unchecked(idx - 1);
                *ret.get_unchecked_mut(idx) *= prev;
            }
        }

        prev = 1;
        for idx in (0..len - 1).rev() {
            unsafe {
                prev *= nums.get_unchecked(idx + 1);
                *ret.get_unchecked_mut(idx) *= prev;
            }
        }

        ret
    }
}
