struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut mask = 0x40000000;
        let mut ans = 0;

        while mask > 0 {
            let l_mask = left & mask;
            let r_mask = right & mask;
            if l_mask != r_mask {
                return ans;
            } else if l_mask != 0 && r_mask != 0 {
                ans |= mask;
            }
            mask >>= 1;
        }
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
    }
}
