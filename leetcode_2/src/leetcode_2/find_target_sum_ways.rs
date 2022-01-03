struct Solution;


// dp is good
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        if target.abs() > total {
            return 0;
        }

        let n = total as usize * 2 + 1;
        let index = |num: i32| (num + total) as usize;
        let mut dp = vec![0; n];
        dp[index(nums[0])] = 1;
        dp[index(-nums[0])] += 1;
        for &num in &nums[1..] {
            let mut next = vec![0; n];
            for sum in -total..total + 1 {
                let count = dp[index(sum)];
                if count > 0 {
                    next[index(sum + num)] += count;
                    next[index(sum - num)] += count;
                }
            }
            dp = next;
        }
        dp[index(target)]
    }
}

// My DFS performance is very bad.
// impl Solution {
//     pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
//         fn inner(nums: &[i32], limit: i32, target: i32, ans: &mut i32) {
//             if target > limit || target < -limit {
//                 return;
//             }
//             if nums.is_empty() {
//                 if target == 0 {
//                     *ans += 1;
//                 }
//                 return;
//             }
//             let val = nums[0];
//             inner(&nums[1..], limit - val, target - val, ans);
//             inner(&nums[1..], limit - val, target + val, ans);
//         }

//         let mut ans = 0;
//         let limit = nums.iter().sum();
//         inner(&nums, limit, target, &mut ans);
//         ans
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }
    #[test]
    fn test_2() {
        assert_eq!(1, Solution::find_target_sum_ways(vec![1], 1));
    }
}
