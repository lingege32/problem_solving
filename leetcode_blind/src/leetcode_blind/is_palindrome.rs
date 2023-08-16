struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let trim = s
            .as_bytes()
            .iter()
            .map(|&x| {
                if x >= 'a' as u8 && x <= 'z' as u8 {
                    x & 0x5F
                } else {
                    x
                }
            })
            .filter(|&x| (x >= 'A' as u8 && x <= 'Z' as u8) || (x >= '0' as u8 && x <= '9' as u8))
            .collect::<Vec<_>>();
        if !trim.is_empty() {
            let mut left = 0;
            let mut right = trim.len() - 1;
            while left < right {
                if trim[left] != trim[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "A man, a plan, a canal: Panama".to_owned();
        let ans = true;
        assert_eq!(Solution::is_palindrome(s), ans);
    }
}
