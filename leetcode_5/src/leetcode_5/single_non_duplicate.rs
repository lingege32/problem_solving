struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn single_non_duplicate2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut left = 0;
        let mut right = nums.len();

        while right > left + 1 {
            let mid = (right + left) / 2;
            if nums[mid] == nums[mid ^ 1] {
                left = (mid | 1) + 1;
            } else {
                right = mid | 1;
            }
        }
        nums[left]
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let idx = Self::helper(&nums);
        nums[idx]
    }
    fn helper(nums: &[i32]) -> usize {
        let (mut left, mut right) = (0, nums.len());
        while right - left > 2 {
            let mid = left + (right - left) / 2;
            if nums[mid - 1] != nums[mid] && nums[mid] != nums[mid + 1] {
                return mid;
            } else {
                let new_mid = if nums[mid - 1] == nums[mid] {
                    mid - 1
                } else {
                    mid
                };
                if new_mid % 2 == 0 {
                    if new_mid == left {
                        left = new_mid + 2;
                    } else {
                        left = new_mid;
                    }
                } else {
                    right = new_mid;
                }
            }
        }

        if left > 0 && nums[left - 1] != nums[left] {
            return left;
        } else {
            return right - 1;
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        let ans = 2;
        assert_eq!(ans, Solution::single_non_duplicate2(nums));
    }
    #[test]
    fn test_2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        let ans = 10;
        assert_eq!(ans, Solution::single_non_duplicate2(nums));
    }
    #[test]
    fn test_3() {
        let nums = vec![1, 1, 2, 3, 3];
        let ans = 2;
        assert_eq!(ans, Solution::single_non_duplicate2(nums));
    }
}
