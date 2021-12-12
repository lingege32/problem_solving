struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums2.len() < nums1.len() {
            Self::find_median_sorted_arrays(nums2, nums1)
        } else {
            let target = (nums1.len() + nums2.len() + 1) / 2;
            let (mut l, mut r) = (0usize, nums1.len());
            while l < r {
                let m1 = (l + r) / 2;
                let m2 = target - m1;
                println!("nums1: {:?}", nums1);
                println!("nums2: {:?}", nums2);
                println!("l: {}, r: {}, m1: {}, m2: {}", l, r, m1, m2);
                if nums1[m1] < nums2[m2 - 1] {
                    l = m1 + 1;
                } else {
                    r = m1;
                }
            }
            let m1 = l;
            let m2 = target - m1;
            println!("nums1: {:?}", nums1);
            println!("nums2: {:?}", nums2);
            println!("m1: {}, m2: {}", m1, m2);
            let first_m = if m1 == 0 {
                nums2[m2 - 1]
            } else {
                if m2 == 0 {
                    nums1[m1 - 1]
                } else {
                    nums1[m1 - 1].max(nums2[m2 - 1])
                }
            };
            if (nums1.len() + nums2.len()) % 2 == 0 {
                let second_m = if m1 == nums1.len() {
                    nums2[m2]
                } else {
                    if m2 == nums2.len() {
                        nums1[m1]
                    } else {
                        nums1[m1].min(nums2[m2])
                    }
                };
                (first_m + second_m) as f64 / 2.0
            } else {
                first_m as f64
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(2.0, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn test_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(2.5, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn test_3() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(0.0, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn test_4() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(1.0, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn test_5() {
        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(2.0, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn test_6() {
        let nums1 = vec![0,0,0,0,0];
        let nums2 = vec![-1,0,0,0,0,0,1];
        assert_eq!(0.0, Solution::find_median_sorted_arrays(nums1, nums2));
    }
}
