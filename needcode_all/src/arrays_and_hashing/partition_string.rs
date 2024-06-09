struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn partition_string(s: String) -> i32 {
        let mut hs = [0; 26];
        let mut ret = 1;
        for c in s.chars().map(|c| c as usize - 'a' as usize) {
            if hs[c] != 0 {
                ret += 1;
                hs = [0; 26];
            }
            hs[c] = 1;
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abacaba".to_owned();
        let ans = 4;
        assert_eq!(ans, Solution::partition_string(s));
    }
}
