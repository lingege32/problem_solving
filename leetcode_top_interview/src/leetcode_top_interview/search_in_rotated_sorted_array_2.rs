struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Self::inner(&nums, target)
    }

    fn inner(nums: &[i32], target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let l = nums[0];
        let r = *nums.last().unwrap();
        if l == target || r == target {
            return true;
        }
        if nums.len() == 1 {
            return false;
        }

        if l < r {
            return nums.binary_search(&target).is_ok();
        }

        let mid = nums.len() / 2;
        let m = nums[mid];
        if m == target {
            return true;
        }
        if m > l {
            if target > l && target < m {
                nums[..mid].binary_search(&target).is_ok()
            } else {
                Self::inner(&nums[mid + 1..], target)
            }
        } else if m == l && m == r {
            Self::inner(&nums[..mid], target) || Self::inner(&nums[mid + 1..], target)
        } else if m == l {
            if target > m || target < l {
                Self::inner(&nums[mid + 1..], target)
            } else {
                false
            }
        } else if m == r {
            if target < m || target > r {
                Self::inner(&nums[..mid], target)
            } else {
                false
            }
        } else
        /* m < l */
        {
            if target > m && target < l {
                nums[mid + 1..].binary_search(&target).is_ok()
            } else {
                Self::inner(&nums[..mid], target)
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 0, 1, 1, 1];
        let target = 0;
        let ans = true;
        assert_eq!(ans, Solution::search(nums, target));
    }
    #[test]
    fn test_12() {
        let nums = vec![2, 2, 2, 0, 1];
        let target = 0;
        let ans = true;
        assert_eq!(ans, Solution::search(nums, target));
    }
}
