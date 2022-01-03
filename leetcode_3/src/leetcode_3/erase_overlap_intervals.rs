struct Solution {}

use std::collections::BTreeMap;

impl Solution {
    #[allow(dead_code)]
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for interval in intervals {
            match map.get_mut(&interval[1]) {
                None => {
                    map.insert(interval[1], interval[0]);
                }
                Some(x) => {
                    if *x < interval[0] {
                        *x = interval[0];
                    }
                }
            }
        }
        println!("{:?}", map);

        let mut count = 1;
        let mut min_end = *map.keys().next().unwrap();
        for (&end, &start) in map.iter() {
            if start >= min_end {
                count += 1;
                min_end = end;
            }
        }
        // println!("count:{}", count);
        n as i32 - count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = [[1, 2], [2, 3], [3, 4], [1, 3]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let ans = 1;
        assert_eq!(ans, Solution::erase_overlap_intervals(intervals));
    }

    #[test]
    fn test_12() {
        let intervals = [[1, 5], [6, 7], [3, 7]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let ans = 1;
        assert_eq!(ans, Solution::erase_overlap_intervals(intervals));
    }
}
