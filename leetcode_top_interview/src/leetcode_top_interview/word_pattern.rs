struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut s_to_p = HashMap::new();
        let mut p_to_s = [""; 26];
        let word_len = s.split(" ").count();
        if word_len != pattern.len() {
            return false;
        }
        for (p, w) in pattern.chars().zip(s.split(" ")) {
            let mapping_p = *s_to_p.entry(w).or_insert(p);
            if mapping_p != p {
                return false;
            }
            let idx = p as usize - 'a' as usize;
            let mapping_word = p_to_s[idx];
            if mapping_word == "" {
                p_to_s[idx] = w;
            } else if mapping_word != w {
                return false;
            }
        }
        true
    }
}
