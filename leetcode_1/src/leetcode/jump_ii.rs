// other's 100% Solutionimpl 

struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur_pos : usize = 0;
        let mut next_max_pos : usize = cur_pos + nums[cur_pos] as usize;
        next_max_pos = next_max_pos.min(nums.len() - 1);
        let mut count : i32 = 0;
        while cur_pos != nums.len() - 1 {
            let mut cur_max_pos : usize = 0;
            println!("cur_pos={}, cur_max_pos={}, next_max_pos: {}", cur_pos, cur_max_pos, next_max_pos);
            for i in (cur_pos+1)..=next_max_pos {
                cur_max_pos = cur_max_pos.max(i + nums[i] as usize);
                cur_max_pos = cur_max_pos.min(nums.len() - 1);
            }
            println!("-- cur_pos={}, cur_max_pos={}, next_max_pos: {}", cur_pos, cur_max_pos, next_max_pos);

            cur_pos = next_max_pos;
            next_max_pos = cur_max_pos;
            count+=1;
        }
        count
    }
}

// impl Solution {
//     pub fn jump(nums: Vec<i32>) -> i32 {
//         let mut dp = vec![0; nums.len()];
//         let len = nums.len();
//         (0..len - 1).rev().for_each(|idx| {
//             dp[idx] = if idx + nums[idx] as usize >= len - 1 {
//                 1
//             } else {
//                 dp[idx + 1..=idx + nums[idx] as usize]
//                     .iter()
//                     .filter(|x| **x != 0)
//                     .min()
//                     .map_or(0, |x| *x + 1)
//             }
//         });
//         dp[0]
//     }
// }
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [2, 3, 1, 1, 4].to_vec();
        assert_eq!(2, Solution::jump(nums));
    }

    #[test]
    fn test_2() {
        let nums = [2, 0, 2, 4, 6, 0, 0, 3].to_vec();
        assert_eq!(3, Solution::jump(nums));
    }

    #[test]
    fn test_3() {
        let nums = [5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0].to_vec();
        assert_eq!(3, Solution::jump(nums));
    }
}
