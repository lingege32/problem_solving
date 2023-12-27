struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() - k as usize;
        // implement nth_element in rust
        Self::nth_element(&mut nums, n)
    }
    fn nth_element(nums: &mut [i32], n: usize) -> i32 {
        let mut len = nums.len();
        let (mut left, mut right) = (0, nums.len() - 1);
        loop {
            if len == 0 || right - left + 1 <= 3 {
                nums[left..=right].sort_unstable();
                return nums[n];
            }
            len /= 2;
            let pivot_idx = Self::partition(nums, left, right);
            // println!("nums: {:?}", nums);
            // println!("pivot_idx: {pivot_idx}");
            if pivot_idx == n {
                return nums[pivot_idx];
            }
            if pivot_idx < n {
                left = pivot_idx + 1
            } else {
                right = pivot_idx - 1;
            }
        }
    }

    fn place_middle_at_last(nums: &mut [i32], last: usize) -> i32 {
        let v1 = nums[last - 2];
        let v2 = nums[last - 1];
        let v3 = nums[last - 0];
        if v1 < v2 {
            if v3 < v1 {
                nums.swap(last - 2, last);
            } else {
                if v2 < v3 {
                    nums.swap(last - 1, last);
                }
            }
        } else {
            if v3 < v2 {
                nums.swap(last - 1, last);
            } else {
                if v3 >= v1 {
                    nums.swap(last - 2, last);
                }
            }
        }
        nums[last]
    }

    fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
        let pivot = Self::place_middle_at_last(nums, right);
        let mut stored_idx = left;
        for i in left..right {
            if nums[i] < pivot {
                nums.swap(i, stored_idx);
                stored_idx += 1;
            }
        }
        nums.swap(stored_idx, right);
        stored_idx
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let ans = 5;
        assert_eq!(ans, Solution::find_kth_largest(nums, k));
    }
    #[test]
    fn test_12() {
        let nums = vec![3, 3, 3, 3, 3, 3, 3, 3, 3];
        let k = 1;
        let ans = 3;
        assert_eq!(ans, Solution::find_kth_largest(nums, k));
    }
    #[test]
    fn test_123() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let ans = 4;
        assert_eq!(ans, Solution::find_kth_largest(nums, k));
    }
}
