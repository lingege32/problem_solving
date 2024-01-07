struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len() / 2;
        let nums1 = Self::unique(nums1);
        let nums2 = Self::unique(nums2);
        let n1 = nums1.len();
        let n2 = nums2.len();
        if nums1.len() <= n && nums2.len() <= n {
            return nums1.union(&nums2).count() as i32;
        }
        let remove_n1 = if n1 > n { n1 - n } else { 0 };
        let remove_n2 = if n2 > n { n2 - n } else { 0 };
        let inter = nums1.intersection(&nums2).map(|x| *x).collect::<Vec<_>>();
        let inter_len = inter.len();
        let last_remove = if remove_n1 + remove_n2 > inter_len {
            remove_n1 + remove_n2 - inter_len
        } else {
            0
        };
        (nums1.union(&nums2).count() - last_remove) as i32
    }
    fn unique(nums: Vec<i32>) -> HashSet<i32> {
        nums.into_iter().collect::<HashSet<_>>()
    }
}
