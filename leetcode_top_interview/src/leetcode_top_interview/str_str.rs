struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h = haystack.as_bytes();
        let n = needle.as_bytes();

        let hlen = h.len();
        let nlen = n.len();

        if hlen < nlen {
            return -1;
        }

        let c = n[0];
        for idx in h[0..=hlen - nlen]
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == c)
            .map(|(idx, _)| idx)
        {
            if &h[idx + 1..idx + nlen] == &n[1..] {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let haystack = "sadbutsad".to_string();

        let needle = "sad".to_string();
        assert_eq!(0, Solution::str_str(haystack, needle));
    }
}
