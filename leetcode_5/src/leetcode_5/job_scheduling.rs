struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut dp = vec![0; n + 1];
        let mut index = (0..n).collect::<Vec<_>>();
        let mut end_time = end_time;
        index.sort_unstable_by(|l, r| end_time[*l].cmp(&end_time[*r]));
        end_time.sort_unstable();
        for idx in 1..=n {
            let prev_idx = Self::upper_bound(&end_time, start_time[index[idx - 1]]);
            dp[idx] = dp[idx - 1].max(dp[prev_idx] + profit[index[idx-1]]);
        }
        // println!("dp: {:?}",dp);
        dp[n]
    }
    fn upper_bound(vec: &[i32], val: i32) -> usize {
        let (mut left, mut right) = (0, vec.len());
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_val = vec[mid];
            if val < mid_val {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        // println!("vec: {:?}, val: {val}, upper_bound: {left}",vec);
        left
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let start = vec![1, 2, 3, 3];
        let end = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        let ans = Solution::job_scheduling(start, end, profit);
        let golden = 120;
        assert_eq!(golden, ans);
    }
    #[test]
    fn test_2() {
        let start = vec![1,2,3,4,6];
        let end = vec![3,5,10,6,9];
        let profit = vec![20,20,100,70,60];
        let ans = Solution::job_scheduling(start, end, profit);
        let golden = 150;
        assert_eq!(golden, ans);
    }
}
