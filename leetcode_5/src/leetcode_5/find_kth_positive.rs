struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let p = Self::linear_search(&arr, k);
        if p == 0 {
            k
        } else {
            k + p as i32
        }
    }
    #[allow(dead_code)]
    fn binary_search(arr: &[i32], k: i32) -> usize {
        let (mut left, mut right) = (0, arr.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if (arr[mid] - (mid as i32 + 1)) < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left;
    }
    fn linear_search(arr: &[i32], k: i32) -> usize {
        for (i, &v) in arr.iter().enumerate() {
            if v - (i as i32 + 1) >= k {
                return i;
            }
        }
        arr.len()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        assert_eq!(9, Solution::find_kth_positive(arr, k));
    }
}
