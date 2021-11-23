struct Solution {}

// best performance
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(arr: Vec<i32>, target: i32) -> i32 {
    let mut map = HashMap::with_capacity(arr.len());
    map.insert(0i64, 1usize); // sum->count

    let mut count = 0usize;
    let mut sum = 0i64;
    for &elem in &arr {
        sum += elem as i64;
        count += map.get(&(sum - target as i64)).unwrap_or(&0);
        *map.entry(sum).or_insert(0) += 1;
    }

    count as i32
    }
}


// fuck bad performance
// impl Solution {
//     pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
//         let mut sum_vec = vec![0; nums.len()];
//         sum_vec[0] = nums[0];
//         for idx in 1..nums.len() {
//             sum_vec[idx] = sum_vec[idx - 1] + nums[idx];
//         }
//         let mut count = 0;
//         for &val in &sum_vec {
//             if val == k {
//                 count += 1;
//             }
//         }
//         for idx in 0..nums.len() {
//             for idx2 in idx + 1..nums.len() {
//                 if sum_vec[idx2] - sum_vec[idx] == k {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

#[cfg(test)]
mod test_super {
    use std::os::unix::net::SocketAddr;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    }
    #[test]
    fn test_2() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }
}
