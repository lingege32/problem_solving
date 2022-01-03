struct Solution {}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn is_match_u8(s: &[u8], p: &[u8]) -> bool {
            let (sl, pl) = (s.len(), p.len());
            let mut cur = vec![false; pl + 1];
            for s_idx in 0..=sl {
                let mut pre = cur[0];
                cur[0] = if s_idx == 0 { true } else { false };
                for p_idx in 1..=pl {
                    let tmp = cur[p_idx];
                    cur[p_idx] = if p[p_idx - 1] == '*' as u8 {
                        cur[p_idx - 2]
                            || (s_idx != 0
                                && cur[p_idx]
                                && (s[s_idx - 1] == p[p_idx - 2] || p[p_idx - 2] == '.' as u8))
                    } else {
                        s_idx != 0
                            && pre
                            && (s[s_idx - 1] == p[p_idx - 1] || p[p_idx - 1] == '.' as u8)
                    };
                    pre = tmp;
                }
            }
            *cur.last().unwrap()
        }
        is_match_u8(s.as_bytes(), p.as_bytes())
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "aa".to_string();
        let p = "a".to_owned();
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn test_3() {
        let s = "ab".to_string();
        let p = ".*".to_owned();
        assert!(Solution::is_match(s, p));
    }
    #[test]
    fn test_2() {
        let s = "aab".to_string();
        let p = "c*a*b".to_owned();
        assert!(Solution::is_match(s, p));
    }
    #[test]
    fn test_4() {
        let s = "mississippi".to_string();
        let p = "mis*is*p*.".to_owned();
        assert!(!Solution::is_match(s, p));
    }
}
