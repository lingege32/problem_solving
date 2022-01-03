struct Solution {}

// faster algorithm
impl Solution {
    #[allow(dead_code)]
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let nums: Vec<_> = nums.into_iter().map(|i| i as usize).collect();
        let sum: usize = nums.iter().sum();
        if sum & 0x1 == 0x1 {
            return false;
        }
        let half = sum >> 1;
        let mut sum_map = vec![false; sum + 1];
        sum_map[0] = true;
        let mut max = 0; // <---- the key bottle neck
        for num in nums {
            for i in (0..=max).rev() {
                if sum_map[i] {
                    sum_map[i + num] = true;
                }
            }
            max = max + num;
            if sum_map[half] {
                return true;
            }
        }
        false
    }
}


// impl Solution {
//     #[allow(dead_code)]
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let sum: i32 = nums.iter().sum();
//         if sum & 1 == 1 {
//             false
//         } else {
//             let mut dp: Vec<u8> = vec![0; sum as usize + 1];
//             dp[0] = 1;
//             for num in nums {
//                 for i in (0..=(sum - num)).rev() {
//                     if dp[i as usize] == 1 {
//                         dp[(i + num) as usize] = 1;
//                     }
//                 }
//                 if dp[(sum / 2) as usize] == 1 {
//                     return true;
//                 }
//             }
//             false
//         }
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::can_partition(vec![1, 2, 5]));
    }
}
