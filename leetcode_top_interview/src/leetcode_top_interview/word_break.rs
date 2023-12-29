struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let max_len = word_dict.iter().map(|x| x.len()).max().unwrap();
        let word_dict = word_dict
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for dp_idx in 1..s.len() + 1 {
            let begin = if dp_idx > max_len {
                dp_idx - max_len
            } else {
                0
            };
            for left in begin..dp_idx {
                if dp[left] && word_dict.contains(&s[left..dp_idx]) {
                    dp[dp_idx] = true;
                    break;
                }
            }
        }

        *dp.last().unwrap()
    }

    /* use hs to trim the leaf */
    /* it's faster than version 1 */
    #[allow(dead_code)]
    pub fn word_break_2(s: String, word_dict: Vec<String>) -> bool {
        let word_dict = Self::trim_word_dict(&s, word_dict);
        let mut hs = std::collections::HashSet::<&str>::new();
        Self::inner(&s, &word_dict, &mut hs)
    }

    fn inner<'a>(
        s: &'a str,
        word_dict: &[String],
        hs: &mut std::collections::HashSet<&'a str>,
    ) -> bool {
        if s.is_empty() {
            return true;
        }
        if hs.contains(s) {
            return false;
        }
        for word in word_dict.iter() {
            if let Some(idx) = s.find(word) {
                if Self::inner(&s[0..idx], word_dict, hs)
                    && Self::inner(&s[idx + word.len()..], word_dict, hs)
                {
                    return true;
                }
            }
        }
        hs.insert(s);
        false
    }

    fn trim_word_dict(s: &str, word_dict: Vec<String>) -> Vec<String> {
        word_dict
            .into_iter()
            .filter(|w| s.find(w).is_some())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let word = vec!["leet".to_string(), "code".to_string()];
        let ans = true;
        assert_eq!(ans, Solution::word_break(s, word));
    }
    #[test]
    fn test_12() {
        let s = "catsandog".to_string();
        let word = ["cats", "dog", "sand", "and", "cat"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let ans = false;
        assert_eq!(ans, Solution::word_break(s, word));
    }
    #[test]
    fn test_123() {
        let s = "applepenapple".to_string();
        let word = ["apple", "pen"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let ans = true;
        assert_eq!(ans, Solution::word_break(s, word));
    }
    #[test]
    fn test_1234() {
        let s = "applepenapple".repeat(6000);
        let word = ["apple", "pen"]
            .iter()
            .cycle()
            .take(100)
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let ans = true;

        let before1 = std::time::Instant::now();
        assert_eq!(ans, Solution::word_break(s, word));
        println!("version 1. Elapsed time: {:.2?}", before1.elapsed());
        let s = "applepenapple".repeat(5010);
        let word = ["apple", "pen"]
            .iter()
            .cycle()
            .take(100)
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let before2 = std::time::Instant::now();
        assert_eq!(ans, Solution::word_break_2(s, word));
        println!("version 2. Elapsed time: {:.2?}", before2.elapsed());
    }
}
