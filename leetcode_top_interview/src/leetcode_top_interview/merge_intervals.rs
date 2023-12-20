struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|x| x[0]);
        let mut ret = vec![];
        let mut iter = intervals.into_iter();
        let mut range = iter.next().unwrap();
        for i in iter {
            let l = i[0];
            let r = i[1];
            if l > range[1] {
                ret.push(range);
                range = vec![l, r];
            } else {
                range[1] = range[1].max(r);
            }
        }
        ret.push(range);
        ret
    }
}
