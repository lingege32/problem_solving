struct Solution {
    
}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut cur = max;
        for value in nums {
            if cur < 0 {
                cur = value;
            } else {
                cur += value;
            }
            if cur > max {
                max = cur;
            }
        }
        max
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [-2,1,-3,4,-1,2,1,-5,4];
        assert_eq!(6, Solution::max_sub_array(nums.to_vec()));
    }
}