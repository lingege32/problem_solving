struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let k: usize = k as usize;
        events.sort_unstable();
        let n = events.len();
        let mut dp = vec![vec![0; n + 1]; k + 1];

        for idx in (0..n).rev() {
            let right = events[idx][1];
            let lower_bound = events.partition_point(|event| event[0] <= right);
            for p in 1..=k {
                dp[p][idx] = dp[p][idx + 1].max(events[idx][2] + dp[p - 1][lower_bound]);
            }
        }
        dp[k][0]
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let events = [[1, 2, 4], [3, 4, 3], [2, 3, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let k = 2;
        let ans = 7;
        assert_eq!(ans, Solution::max_value(events, k));
    }
    #[test]
    fn test_2() {
        let events = [[1, 2, 4], [3, 4, 3], [2, 3, 10]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let k = 2;
        let ans = 10;
        assert_eq!(ans, Solution::max_value(events, k));
    }
    #[test]
    fn test_3() {
        let events = [[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let k = 3;
        let ans = 9;
        assert_eq!(ans, Solution::max_value(events, k));
    }
}
