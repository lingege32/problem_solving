struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut arr1 = Vec::with_capacity(len);
        let mut arr2 = Vec::with_capacity(len);
        arr1.push(nums[0]);
        arr2.push(nums[1]);
        for idx in 2..len {
            let e = nums[idx];
            let l = Self::count(&arr1, e);
            let r = Self::count(&arr2, e);
            if l > r {
                arr1.push(e);
            } else if l < r {
                arr2.push(e);
            } else {
                if arr1.len() <= arr2.len() {
                    arr1.push(e);
                } else {
                    arr2.push(e);
                }
            }
        }
        arr1.append(&mut arr2);
        arr1
    }
    fn count(arr: &[i32], k: i32) -> i32 {
        let mut count = 0;
        for &a in arr {
            if a > k {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let nums = vec![5, 14, 3, 1, 2];
        let ans = vec![5, 3, 1, 2, 14];
        assert_eq!(Solution::result_array(nums), ans);
    }
}
