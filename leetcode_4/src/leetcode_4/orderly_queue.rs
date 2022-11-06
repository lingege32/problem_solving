struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn orderly_queue(s: String, k: i32) -> String {
        // Solution reference: https://www.cnblogs.com/grandyang/p/10995474.html
        if k > 1 {
            let mut v8 = s.into_bytes();
            v8.sort_unstable();
            return String::from_utf8(v8).unwrap();
        } else {
            let mut ret = s.clone();
            for i in 1..ret.len() {
                let mut new = s[i..].to_string();
                new.push_str(&s[0..i]);
                ret = ret.min(new);
            }
            ret
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::orderly_queue("cba".to_string(), 1);
        assert_eq!("acb", ans);
    }
}
