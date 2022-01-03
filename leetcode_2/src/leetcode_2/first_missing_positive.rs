struct Solution {}
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut nums = nums;

        for m in &mut nums {
            if *m < 1 || *m > n {
                *m = 0;
            }
        }

        // 假如nums[1] = 3
        // 我們為nums[3 - 1] 加上n+1
        // 以此類推 會為每個idx加上n+1
        // 所以最後如果有缺失的idx那一定是會小於n+1
        for i in 0..n as usize {
            println!("i: {}, nums: {:?}", i, nums);
            if nums[i] % (n + 1) > 0 {
                let ix = (nums[i] % (n + 1) - 1) as usize;
                nums[ix] += n + 1;
            }
        }

        println!("nums: {:?}", nums);
        for i in 0..n as usize {
            if nums[i] <= n {
                return i as i32 + 1;
            }
        }

        n + 1
    }
}
// my poor solution
// use std::collections::HashSet;
// impl Solution {
//     pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
//         let mut max = None;
//         let mut set = HashSet::new();
//         for &i in nums.iter() {
//             if i > 0 {
//                 match max {
//                     Some(x) => {
//                         if x < i {
//                             max = Some(i);
//                         }
//                     }
//                     None => {
//                         max = Some(i);
//                     }
//                 }
//                 set.insert(i);
//             }
//         }
//         (1..).find(|x| !set.contains(x)).unwrap()
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 0];
        assert_eq!(3, Solution::first_missing_positive(nums));
    }
    #[test]
    fn test_2() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(2, Solution::first_missing_positive(nums));
    }
    #[test]
    fn test_3() {
        let nums = vec![7, 8, 9, 11, 12];
        assert_eq!(1, Solution::first_missing_positive(nums));
    }
}
