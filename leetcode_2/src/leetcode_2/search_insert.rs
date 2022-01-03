struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (left + right) / 2;
            if mid == left {
                break;
            }
            if nums[mid] < target {
                left = mid;
            } else {
                right = mid;
            }
        }
        if nums[left] < target {
            (left + 1) as i32
        } else {
            left as i32
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    }
    #[test]
    fn test_2() {
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    }
    #[test]
    fn test_3() {
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    }
}
