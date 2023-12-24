struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left_intervals = vec![];
        let mut right_intervals = vec![];
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
