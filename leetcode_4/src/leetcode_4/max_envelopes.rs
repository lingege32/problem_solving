struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        Self::solution1(envelopes)
    }
    fn solution1(mut envelopes: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
        let mut dp = Vec::new();
        for env in envelopes {
            let height = env[1];
            match dp.binary_search(&height) {
                Ok(idx) => {
                    dp[idx] = height;
                }
                Err(idx) => {
                    if idx == dp.len() {
                        dp.push(height);
                    } else {
                        dp[idx] = height;
                    }
                }
            }
        }

        dp.len() as i32
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let envelopes = [[5, 4], [6, 4], [6, 7], [2, 3]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let ans = 3;
        assert_eq!(ans, Solution::max_envelopes(envelopes));
    }

    #[test]
    fn test_2() {
        let envelopes = [[2, 7], [2, 6]].iter().map(|x| x.to_vec()).collect();
        let ans = 1;
        assert_eq!(ans, Solution::max_envelopes(envelopes));
    }
}
