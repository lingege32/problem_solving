struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.chars();
        let mut sc = match s_iter.next() {
            Some(c) => c,
            None => {
                return true;
            }
        };

        for c in t.chars() {
            if c == sc {
                sc = match s_iter.next() {
                    Some(c) => c,
                    None => {
                        return true;
                    }
                };
            }
        }
        false
    }
}
