struct Solution {}

// use heap to find nth_element
// use std::collections::BinaryHeap;
// use std::cmp::Reverse;
// impl Solution {
//     pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
//         let mut k = k as usize;
//         let mut minheap = BinaryHeap::with_capacity(k);
//         nums.into_iter().for_each(|i| minheap.push(Reverse(i)));
//         while minheap.len() > k as usize { 
//             minheap.pop();  
//         }
//         minheap.peek().unwrap().0
//     }
// }


impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        fn kth_elements(nums: &mut [i32], k: i32) -> i32 {
            if nums.len() < 4 {
                nums.sort_unstable();
                nums[k as usize]
            } else {
                fn move_pivot_to_front(nums: &mut [i32]) -> i32 {
                    let mid = nums.len() / 2;
                    if nums[0] >= nums[mid] {
                        nums.swap(0, mid);
                    }
                    if nums[mid] >= nums[nums.len() - 1] {
                        nums.swap(mid, nums.len() - 1);
                    }
                    if nums[0] < nums[mid] {
                        nums.swap(0, mid);
                    }
                    nums[0]
                }
                let pivot = move_pivot_to_front(nums);
                let mut left = 1;
                let mut right = nums.len();
                let cut = loop {
                    while nums[left] < pivot {
                        left += 1;
                    }
                    right -= 1;
                    while pivot < nums[right] {
                        right -= 1;
                    }
                    if !(left < right) {
                        break left as i32;
                    }
                    nums.swap(left, right);
                    left += 1;
                };
                if cut > k {
                    kth_elements(&mut nums[..cut as usize], k)
                } else {
                    kth_elements(&mut nums[(cut as usize)..], k - cut)
                }
            }
        }
        let mut nums = nums;
        let k = nums.len() as i32 - k;
        kth_elements(&mut nums, k)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
