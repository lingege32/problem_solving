struct Solution();
impl Solution {
    #[allow(dead_code)]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|lhs, rhs| match lhs[0].cmp(&rhs[0]) {
            std::cmp::Ordering::Equal => lhs[1].cmp(&rhs[1]),
            n => n,
        });
        let mut res = vec![];
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];
        for i in &intervals[1..] {
            if i[0] > right {
                res.push(vec![left, right]);
                left = i[0];
            }
            if i[1] > right {
                right = i[1];
            }
        }
        res.push(vec![left, right]);
        res
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::merge(vec![vec![1, 4], vec![4, 5]])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        );
    }
}
