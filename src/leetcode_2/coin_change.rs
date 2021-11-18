struct Solution {}
// DFS will exceed time limit....
// impl Solution {
//     pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
//         let mut coins = coins;
//         coins.sort_unstable_by(|a, b| b.cmp(a));
//         fn inner(coins: &[i32], amount: i32, count: i32, min: &mut i32) {
//             // println!("coins: {:?}, amount: {}, count: {}, min: {}", coins, amount, count, min);
//             if amount == 0 {
//                 if count < *min {
//                     *min = count;
//                 }
//             } else if !coins.is_empty() {
//                 let q = amount / coins[0];
//                 for take in (0..=q).rev() {
//                     let left = amount - coins[0] * take;
//                     let n_idx = coins[1..]
//                         .iter()
//                         .enumerate()
//                         .find(|(_, &val)| val <= left)
//                         .map(|(idx, _)| idx + 1)
//                         .unwrap_or(coins.len());
//                     // println!("coins[0]: {}, take: {}, left: {}, n_idx: {}",coins[0], take, left, n_idx);
//                     inner(&coins[n_idx..], left, count + take, min)
//                 }
//             }
//         }
//         let mut min = i32::MAX;
//         inner(&coins, amount, 0, &mut min);
//         if min == i32::MAX {
//             -1
//         } else {
//             min
//         }
//     }
// }

// DP!
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; amount as usize +1];
        dp[0] = 0;
        for i in 1..=amount {
            let mut current_min = i32::MAX;
            for &coin in coins.iter() {
                if i - coin >= 0 {
                    current_min = std::cmp::min(current_min, dp[(i - coin) as usize]);
                }
            }

            if current_min != i32::MAX {
                dp[i as usize] = current_min + 1;
            }
        }
        if dp[amount as usize] == i32::MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::coin_change(vec![1], 0));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::coin_change(vec![1], 1));
    }
    #[test]
    fn test_5() {
        assert_eq!(2, Solution::coin_change(vec![1], 2));
    }
    #[test]
    fn test_6() {
        assert_eq!(20, Solution::coin_change(vec![83, 186, 408, 419], 6249));
    }
}
