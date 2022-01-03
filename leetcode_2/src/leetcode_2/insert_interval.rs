struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left_intervals = vec![];
        let mut right_intervals= vec![];
        let (mut new_left, mut new_right) = (new_interval[0], new_interval[1]);
        for a in intervals.into_iter() {
            let (left, right) = (a[0], a[1]);
            if right < new_left {
                left_intervals.push(a);
            } else if left > new_right {
                right_intervals.push(a);
            } else {
                new_left = new_left.min(left);
                new_right = new_right.max(right);
            }
        }
        left_intervals.push(vec![new_left, new_right]);
        left_intervals.append(&mut right_intervals);
        left_intervals
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = [[1, 3], [6, 9]].iter().map(|x| x.to_vec()).collect();
        let ans: Vec<Vec<i32>> = [[1, 5], [6, 9]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(ans, Solution::insert(intervals, vec![2, 5]));
    }
    #[test]
    fn test_2() {
        let intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let ans: Vec<Vec<i32>> = [[1, 2], [3, 10], [12, 16]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(ans, Solution::insert(intervals, vec![4, 8]));
    }
    #[test]
    fn test_3() {
        let intervals = vec![];
        let ans: Vec<Vec<i32>> = [[5, 7]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(ans, Solution::insert(intervals, vec![5, 7]));
    }
    #[test]
    fn test_4() {
        let intervals = [[1, 5]].iter().map(|x| x.to_vec()).collect();
        let ans: Vec<Vec<i32>> = [[1, 5]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(ans, Solution::insert(intervals, vec![2, 3]));
    }
    #[test]
    fn test_5() {
        let intervals = [[1, 5]].iter().map(|x| x.to_vec()).collect();
        let ans: Vec<Vec<i32>> = [[1, 7]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(ans, Solution::insert(intervals, vec![2, 7]));
    }
}
