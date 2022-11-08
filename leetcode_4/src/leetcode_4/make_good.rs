struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn make_good(s: String) -> String {
        let ret = Self::inner(s.into_bytes());
        return unsafe { String::from_utf8_unchecked(ret) };
    }
    fn inner(mut s: Vec<u8>) -> Vec<u8> {
        while let Some(idx) = Self::helper_find_bad(&s) {
            s = Self::helper_trim(s, idx);
        }
        s
    }
    fn helper(c1: u8, c2: u8) -> bool {
        let new_c1 = if c1 < 97 { c1 + 32 } else { c1 - 32 };
        let ret = new_c1 == c2;
        ret
    }
    fn helper_find_bad(s: &[u8]) -> Option<usize> {
        s.windows(2)
            .enumerate()
            .find(|(_, w)| Self::helper(w[0], w[1]))
            .map(|(idx, _)| idx)
    }
    fn helper_trim(mut s: Vec<u8>, idx: usize) -> Vec<u8> {
        let (mut begin, mut end) = (idx, idx + 1);
        while Self::helper(s[begin], s[end]) && begin != 0 && end + 1 != s.len() {
            begin -= 1;
            end += 1;
        }
        if !Self::helper(s[begin], s[end]) {
            begin += 1;
            end -= 1;
        }
        let dis = end - begin + 1;
        s[begin..].rotate_left(dis);
        s.resize(s.len() - dis, 0 /* unused */);
        s
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = "leEeetcode".to_owned();
        assert_eq!(Solution::make_good(a), "leetcode".to_owned());
    }
    #[test]
    fn test_2() {
        let a = "abBAcC".to_owned();
        assert_eq!(Solution::make_good(a), "".to_owned());
    }
}
