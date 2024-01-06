struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let upper_bound = match candidates.binary_search(&target) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        };

        let mut ret = vec![];
        let mut cur = vec![];
        Self::combine(&candidates[..upper_bound], target, &mut cur, &mut ret);
        ret
    }

    fn combine(cand: &[i32], target: i32, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ret.push(cur.clone());
            return;
        }
        for (idx, &val) in cand.iter().enumerate() {
            if val > target {
                break;
            }
            cur.push(val);
            Self::combine(&cand[idx..], target - val, cur, ret);
            cur.pop();
        }
    }
}
