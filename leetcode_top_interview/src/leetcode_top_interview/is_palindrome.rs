struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|x| x.is_alphabetic() || x.is_alphanumeric())
            .map(|x| if x.is_alphabetic() { x as u8 | 0x20 } else { x as u8})
            .collect::<Vec<_>>();
        if s.is_empty() {
            return true;
        }

        let (mut left, mut right) = (0usize, s.len() - 1);

        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(true, Solution::is_palindrome(s));
    }
    #[test]
    fn test_2() {
        let s = "0P".to_string();
        assert_eq!(false, Solution::is_palindrome(s));
    }
}
