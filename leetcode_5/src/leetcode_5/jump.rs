struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur_pos: usize = 0;
        let mut next_max_pos: usize = cur_pos + nums[cur_pos] as usize;
        next_max_pos = next_max_pos.min(nums.len() - 1);
        let mut count: i32 = 0;
        while cur_pos != nums.len() - 1 {
            let mut cur_max_pos: usize = 0;
            for i in (cur_pos + 1)..=next_max_pos {
                cur_max_pos = cur_max_pos.max(i + nums[i] as usize);
                cur_max_pos = cur_max_pos.min(nums.len() - 1);
            }
            // println!("cur_pos={}, cur_max_pos={}", cur_pos, cur_max_pos);

            cur_pos = next_max_pos;
            next_max_pos = cur_max_pos;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 1, 1, 4, 1, 1, 1, 2, 1, 1];
        assert_eq!(4, Solution::jump(nums));
    }
}
