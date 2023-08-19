struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        loop {
            let mid = (l + r) >> 1;
            match (nums[mid] < nums[l], nums[mid] > nums[r]) {
                (true, _) => r = mid,
                (_, true) => l = mid + 1,
                _ => break,
            }
        }

        nums[l]
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 5, 1, 2];
        let ans = 1;
        assert_eq!(ans, Solution::find_min(nums));
    }
    #[test]
    fn test_2() {
        let nums = vec![2, 1];
        let ans = 1;
        assert_eq!(ans, Solution::find_min(nums));
    }
}
