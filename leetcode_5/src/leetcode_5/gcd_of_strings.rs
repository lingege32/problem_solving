struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let (str1_len, str2_len) = (str1.len(), str2.len());
        let gcd_value = if str1_len < str2_len {
            Self::gcd(str2_len, str1_len)
        } else {
            Self::gcd(str1_len, str2_len)
        };
        let d = &str1[0..gcd_value];
        if Self::check_divisor(&str1, d) && Self::check_divisor(&str2, d) {
            return unsafe { std::str::from_utf8_unchecked(d).to_owned() };
        }
        "".to_owned()
    }

    fn check_divisor(dividend: &[u8], divisor: &[u8]) -> bool {
        let len = divisor.len();
        for index in (0..dividend.len()).step_by(len) {
            if &dividend[index..index + len] != divisor {
                return false;
            }
        }
        true
    }
    fn gcd(big: usize, small: usize) -> usize {
        if small == 0 {
            big
        } else {
            Self::gcd(small, big % small)
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let str1 = "ABCABC".to_owned();
        let str2 = "ABC".to_owned();
        let ans = "ABC".to_owned();
        assert_eq!(ans, Solution::gcd_of_strings(str1, str2));
    }
    #[test]
    fn test_2() {
        let str1 = "LEET".to_owned();
        let str2 = "CODE".to_owned();
        let ans = "".to_owned();
        assert_eq!(ans, Solution::gcd_of_strings(str1, str2));
    }
    #[test]
    fn test_3() {
        let str1 = "ABABAB".to_owned();
        let str2 = "ABAB".to_owned();
        let ans = "AB".to_owned();
        assert_eq!(ans, Solution::gcd_of_strings(str1, str2));
    }
}
