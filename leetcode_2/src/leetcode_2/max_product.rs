struct Solution {}
impl Solution {
    // Kadane's algorithm
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut best = nums[0];
        let mut local = 0;

        for n in nums.iter().copied() {
            local = if local == 0 { 1 } else { local };
            local *= n;
            best = best.max(local);
        }

        local = 0;
        for n in nums.iter().copied().rev() {
            local = if local == 0 { 1 } else { local };
            local *= n;
            best = best.max(local);
        }

        best
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let nums = [2, 3, 0, -2, 4].to_vec();
        assert_eq!(6, Solution::max_product(nums));
    }
    #[test]
    fn test_2() {
        let nums = [-2, 0, -1].to_vec();
        assert_eq!(0, Solution::max_product(nums));
    }
    #[test]
    fn test_3() {
        let nums = [3, -1, 4].to_vec();
        assert_eq!(4, Solution::max_product(nums));
    }
}
