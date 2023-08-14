struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        Self::got_array(s) == Self::got_array(t)
    }
    fn got_array(s: String) -> [usize; 26] {
        let mut ret = [0; 26];
        for c in s.chars() {
            ret[c as usize - 'a' as usize] += 1;
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let ans = true;
        assert_eq!(ans, Solution::is_anagram(s, t));
    }

    #[test]
    fn test_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let ans = false;
        assert_eq!(ans, Solution::is_anagram(s, t));
    }
}
