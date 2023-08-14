struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ret = vec![1; len];
        let mut prev = 1;
        for idx in 1..len {
            let &val = unsafe { nums.get_unchecked(idx - 1) };
            prev *= val;
            unsafe {
                *ret.get_unchecked_mut(idx) *= prev;
            }
        }
        let mut prev = 1;
        for idx in (0..len - 1).rev() {
            let &val = unsafe { nums.get_unchecked(idx + 1) };
            prev *= val;
            unsafe {
                *ret.get_unchecked_mut(idx) *= prev;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let ans = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), ans);
    }
    #[test]
    fn test_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let ans = vec![0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(nums), ans);
    }
}
