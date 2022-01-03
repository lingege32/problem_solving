struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut word_map = HashMap::new();
        word_dict.iter().for_each(|x| {
            let v8 = x.as_bytes();
            word_map.entry(v8[0]).or_insert(vec![]).push(v8);
        });
        for (_, val) in word_map.iter_mut() {
            val.sort_unstable_by(|&lhs, &rhs| rhs.len().cmp(&lhs.len()));
        }

        /* dp */
        fn inner(s: &[u8], hm: &HashMap<u8, Vec<&[u8]>>) -> bool {
            let slen = s.len();
            let mut dp = vec![false; 1 + slen];
            dp[0] = true;
            for idx in 0..s.len() {
                if dp[idx] == false {
                    continue;
                }
                match hm.get(&s[idx]) {
                    Some(patterns) => {
                        for &pattern in patterns {
                            if slen - idx >= pattern.len()
                                && &s[idx..idx + pattern.len()] == pattern
                            {
                                dp[idx + pattern.len()] = true;
                            }
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
            return dp[s.len()];
        }
        inner(s, &word_map)

        /* my Time limite exception code
         * the inner2 may execute multiple times at one location.
         * for s = "aaab"
         * patterns = ["a", "aa"];
         * inner2(s[3..]) is called twice when
         *  - we try "a" -> "aa" and "aa" -> "a"
         * but dynamic programming method always execute once at one location.
         */
        // fn inner2(s: &[u8], hm: &HashMap<u8, Vec<&[u8]>>) -> bool {
        //     if s.is_empty() {
        //         return true;
        //     }
        //     match hm.get(&s[0]) {
        //         Some(v) => {
        //             for &pattern in v {
        //                 if s.len() >= pattern.len() && &s[..pattern.len()] == pattern {
        //                     if inner2(&s[pattern.len()..], hm) {
        //                         return true;
        //                     }
        //                 }
        //             }
        //             false
        //         }
        //         None => false,
        //     }
        // }
        // inner2(s, &word_map)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            ["leet", "code"].iter().map(|x| x.to_string()).collect()
        ));
    }
    #[test]
    fn test_2() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            ["apple", "pen"].iter().map(|x| x.to_string()).collect()
        ));
    }
    #[test]
    fn test_3() {
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            ["cats", "dog", "sand", "and", "cat"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        ));
    }
    #[test]
    fn test_4() {
        assert!(!Solution::word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
            ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        ));
    }
}
