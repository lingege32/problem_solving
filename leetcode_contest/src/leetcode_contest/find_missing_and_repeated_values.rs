struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let len = grid.len() as i32;
        let double = len * len;
        let total = (1 + double) * double / 2;
        let total2 = grid.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>();
        let dis = total - total2;
        let mut hs = HashSet::<i32>::new();
        let mut twice = 0;

        'out: for v in grid.iter() {
            for val in v.iter() {
                if !hs.insert(*val) {
                    twice = *val;
                    break 'out;
                }
            }
        }

        vec![twice, twice + dis]
    }
}
