struct Solution {}
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            false
        } else {
            Self::is_interleave_u8(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
        }
    }
    fn is_interleave_u8(s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;
        for i in 1..=s1.len() {
            if s1[i-1] == s3[i-1] {
                dp[i][0] = true;
            } else {
                break;
            }
        }
        for i in 1..=s2.len() {
            if s2[i-1] == s3[i-1] {
                dp[0][i] = true;
            } else {
                break;
            }
        }
        for dp_s1_idx in 1..=s1.len() {
            for dp_s2_idx in 1..=s2.len() {
                dp[dp_s1_idx][dp_s2_idx] = 
                (dp[dp_s1_idx - 1][dp_s2_idx] && s1[dp_s1_idx - 1] == s3[dp_s1_idx + dp_s2_idx - 1]) ||
                (dp[dp_s1_idx][dp_s2_idx - 1] && s2[dp_s2_idx - 1] == s3[dp_s1_idx + dp_s2_idx - 1]);
            }
        }

        dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_interleave(
            "aabcc".into(),
            "dbbca".into(),
            "aadbbcbcac".into()
        ));
    }
    #[test]
    fn test_2() {
        assert!(!Solution::is_interleave(
            "aabcc".into(),
            "dbbca".into(),
            "aadbbbaccc".into()
        ));
    }
    #[test]
    fn test_3() {
        assert!(Solution::is_interleave("".into(), "".into(), "".into()));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_interleave("bbbbbabbbbabaababaaaabbababbaaabbabbaaabaaaaababbbababbbbbabbbbababbabaabababbbaabababababbbaaababaa".into(), "babaaaabbababbbabbbbaabaabbaabbbbaabaaabaababaaaabaaabbaaabaaaabaabaabbbbbbbbbbbabaaabbababbabbabaab".into(), "babbbabbbaaabbababbbbababaabbabaabaaabbbbabbbaaabbbaaaaabbbbaabbaaabababbaaaaaabababbababaababbababbbababbbbaaaabaabbabbaaaaabbabbaaaabbbaabaaabaababaababbaaabbbbbabbbbaabbabaabbbbabaaabbababbabbabbab".into()));
    }
}
