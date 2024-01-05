struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_min(nums: Vec<i32>) -> i32 {
        Self::inner(&nums)
    }
    fn inner(nums: &[i32]) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);

        while nums[lo] > nums[hi] {
            let mid = lo + (hi - lo) / 2;
            match nums[lo] > nums[mid] {
                true => hi = mid,
                false => lo = mid + 1,
            }
        }

        nums[lo]
    }
}
