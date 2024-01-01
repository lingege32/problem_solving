struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] < *nums.last().unwrap() {
            return nums
                .binary_search(&target)
                .map(|x| (x) as i32)
                .unwrap_or(-1);
        }
        if nums[0] == target {
            return 0;
        }
        if *nums.last().unwrap() == target {
            return nums.len() as i32 - 1;
        }
        if nums.len() == 1 {
            return -1;
        }
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let lv = nums[left];
            let mid = (right - left) / 2 + left;
            let m = nums[mid];
            if m == target {
                return mid as i32;
            }
            if m < lv {
                if m < target && target < lv {
                    return nums[mid..right]
                        .binary_search(&target)
                        .map(|x| (x + mid) as i32)
                        .unwrap_or(-1);
                }
                right = mid;
            } else {
                if target >= lv && target < m {
                    return nums[left..mid]
                        .binary_search(&target)
                        .map(|x| (x + left) as i32)
                        .unwrap_or(-1);
                } else {
                    left = mid + 1;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_12() {
        let nums = vec![2, 4, 7, 9, 0];
        let target = 9;
        let ans = 3;
        assert_eq!(ans, Solution::search(nums, target));
    }
    #[test]
    fn test_123() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;
        let ans = 4;
        assert_eq!(ans, Solution::search(nums, target));
    }
}
