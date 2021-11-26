struct Solution {}

// vector is faster
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![false; n];
        for i in 0..n {
            ans[nums[i] as usize - 1] = true;
        }

        ans.iter()
            .enumerate()
            .filter(|(_, val)| **val == false)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}

// hash set is slow
// impl Solution {
//     #[allow(dead_code)]
//     pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
//         #[allow(unused_imports)]
//         use std::collections::HashSet;
//         let mut count = 0;
//         let mut set = HashSet::new();
//         for &i in nums.iter() {
//             if !set.insert(i) {
//                 count += 1;
//             }
//         }
//         let mut ans = Vec::new();
//         ans.reserve(count);
//         for i in 1..=nums.len() as i32 {
//             if !set.contains(&i) {
//                 ans.push(i);
//             }
//         }

//         ans
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }
}
