struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.rotate_left(m as usize);
        let mut idx1 = n as usize;
        let mut idx2 = 0;
        for idx in 0..nums1.len() {
            if idx1 == nums1.len() || idx2 == nums2.len() {
                break;
            }
            let v1 = Self::get(&nums1, idx1);
            let v2 = Self::get(&nums2, idx2);
            let v = if v1 > v2 {
                idx2 += 1;
                v2
            } else {
                idx1 += 1;
                v1
            };
            unsafe { *nums1.get_unchecked_mut(idx) = v };
        }

        if idx1 == nums1.len() {
            let start = nums1.len() - (n as usize - idx2);
            for val in nums1[start..].iter_mut() {
                *val = nums2[idx2];
                idx2 += 1;
            }
        }
    }
    fn get(nums: &[i32], idx: usize) -> i32 {
        unsafe { *nums.get_unchecked(idx) }
    }
}
