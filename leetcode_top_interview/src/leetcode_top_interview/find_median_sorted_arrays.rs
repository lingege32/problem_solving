struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let (m, n) = (nums1.len(), nums2.len());
        let (mut low, mut high) = (0, m);
        let take = (m + n + 1) / 2;

        while low <= high {
            let partition_1 = (low + high) / 2;
            let partition_2 = take - partition_1;

            let left_1 = if partition_1 == 0 {
                i32::MIN
            } else {
                nums1[partition_1 - 1]
            };
            let right_1 = if partition_1 == m {
                i32::MAX
            } else {
                nums1[partition_1]
            };

            let left_2 = if partition_2 == 0 {
                i32::MIN
            } else {
                nums2[partition_2 - 1]
            };
            let right_2 = if partition_2 == n {
                i32::MAX
            } else {
                nums2[partition_2]
            };

            if left_1 <= right_2 && left_2 <= right_1 {
                if (m + n) % 2 == 0 {
                    return (left_1.max(left_2) + right_1.min(right_2)) as f64 / 2.0;
                } else {
                    return left_1.max(left_2) as f64;
                }
            }
            if left_1 > right_2 {
                high = partition_1 - 1;
            } else {
                low = partition_1 + 1;
            }
        }

        0.0
    }
}
