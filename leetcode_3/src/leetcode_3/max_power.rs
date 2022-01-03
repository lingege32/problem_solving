struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_power(s: String) -> i32 {
        let ss = s.as_bytes();
        let mut max = 1;
        let mut cur = ss[0];
        let mut cur_idx = 0;
        for idx in 1..ss.len() {
            if cur != ss[idx] {
                cur = ss[idx];
                max = max.max(idx - cur_idx);
                cur_idx = idx;
            }
        }
        max = max.max(ss.len() - cur_idx);
        max as i32
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let ans = 2;
        assert_eq!(ans, Solution::max_power(s));
    }

    #[test]
    fn test_2() {
        let s = "abbcccddddeeeeedcba".to_string();
        let ans = 5;
        assert_eq!(ans, Solution::max_power(s));
    }

    #[test]
    fn test_3() {
        let s = "triplepillooooow".to_string();
        let ans = 5;
        assert_eq!(ans, Solution::max_power(s));
    }

    #[test]
    fn test_4() {
        let s = "hooraaaaaaaaaaay".to_string();
        let ans = 11;
        assert_eq!(ans, Solution::max_power(s));
    }

    #[test]
    fn test_5() {
        let s = "tourist".to_string();
        let ans = 1;
        assert_eq!(ans, Solution::max_power(s));
    }
}
