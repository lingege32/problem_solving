struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut hm = nums
            .iter()
            .fold(HashMap::<i32, (i32, i32)>::new(), |mut acc, v| {
                acc.entry(*v).or_insert((0, 0)).0 += 1;
                acc
            });
        let mut ret = 1.max((nums.iter().filter(|x| **x == 1).count() as i32 + 1) / 2);
        for n in nums.into_iter().filter(|x| *x != 1) {
            Self::cal(n, &mut hm, &mut ret);
        }
        ret * 2 - 1
    }
    fn cal(n: i32, hm: &mut HashMap<i32, (i32, i32)>, max: &mut i32) -> i32 {
        match hm.get(&n) {
            Some(v) => {
                if v.1 != 0 {
                    return v.1;
                }
                if v.0 == 1 {
                    return 1;
                }
            }
            None => {
                return 0;
            }
        }
        let tgt_level = Self::cal(n * n, hm, max) + 1;
        hm.get_mut(&n).unwrap().1 = tgt_level;
        *max = tgt_level.max(*max);
        return tgt_level;
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let v = vec![1, 16, 49, 16, 121];
        let ans = 1;
        assert_eq!(ans, Solution::maximum_length(v));
    }
    #[test]
    fn test_2() {
        let v = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024,
        ];
        let ans = 7;
        assert_eq!(ans, Solution::maximum_length(v));
    }
}
