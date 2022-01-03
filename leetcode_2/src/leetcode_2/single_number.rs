struct Solution {
    
}

use std::ops::BitXor;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, i32::bitxor)
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::single_number(vec![2,2,1]));
    }
    #[test]
    fn test_2() {
        assert_eq!(4, Solution::single_number(vec![4,1,2,1,2]));
    }
    #[test]
    fn test_3() {
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}