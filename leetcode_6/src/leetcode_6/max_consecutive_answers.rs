
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let sb = answer_key.as_bytes();
        let mut ans = 0;
        let mut i = 0;
        let mut trues = 0;
        let mut falses = 0;
        for j in 0..sb.len() {
            trues  += (sb[j] == b'T') as i32;
            falses += (sb[j] == b'F') as i32;
            while trues.min(falses) > k {
                trues  -= (sb[i] == b'T') as i32;
                falses -= (sb[i] == b'F') as i32;
                i += 1;
            }
            ans = ans.max(j - i + 1);
        }
        ans as _
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let answer_key = "TTFF".to_string();
        let k = 2;
        let ans = 4;
        assert_eq!(ans, Solution::max_consecutive_answers(answer_key, k));
    }
    #[test]
    fn test_2() {
        let answer_key = "TFFT".to_string();
        let k = 1;
        let ans = 3;
        assert_eq!(ans, Solution::max_consecutive_answers(answer_key, k));
    }
    #[test]
    fn test_3() {
        let answer_key = "TTFTTFTT".to_string();
        let k = 1;
        let ans = 5;
        assert_eq!(ans, Solution::max_consecutive_answers(answer_key, k));
    }
    #[test]
    fn test_4() {
        let answer_key = "TF".to_string();
        let k = 2;
        let ans = 2;
        assert_eq!(ans, Solution::max_consecutive_answers(answer_key, k));
    }
}
