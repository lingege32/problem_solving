struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        let pos = candidates
            .iter()
            .position(|x| *x > target)
            .unwrap_or(candidates.len());
        let mut ans = vec![];
        let mut cur = vec![];
        Self::inner(&candidates[..pos], target, &mut ans, &mut cur);
        ans
    }
    fn inner(candidates: &[i32], target: i32, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
        if target == 0 {
            ans.push(cur.clone());
        } else {
            for (idx, &val) in candidates.iter().enumerate() {
                if val > target {
                    break;
                }
                cur.push(val);
                Self::inner(&candidates[idx..], target - val, ans, cur);
                cur.pop();
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let ans = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(ans, Solution::combination_sum(candidates, target));
    }
}
