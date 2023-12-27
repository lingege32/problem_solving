struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left + 3 < right {
            let mid = (left + right) / 2;
            let (l, m, r) = (nums[mid - 1], nums[mid], nums[mid + 1]);
            if m > l && m > r {
                return mid as i32;
            } else if m > l {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        (left..right)
            .map(|x| (x as i32, nums[x]))
            .max_by_key(|(_, height)| *height)
            .map(|x| x.0)
            .unwrap()
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums =vec![1,2,3,1];
        let ans = 2;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }
    #[test]
    fn test_2() {
        let nums =vec![1];
        let ans = 0;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }
}