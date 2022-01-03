struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn calculate(s: String) -> i32 {
        let mut v: Vec<i32> = Vec::new();
        let mut n = 0;
        let mut prev: u8 = if s.trim_start().starts_with('-') {
            b'-'
        } else {
            b'+'
        };
        for &b in (s + "*1").as_bytes() {
            if b == b' ' {
                continue;
            }
            match b {
                b'0'..=b'9' => {
                    n = n * 10 + (b - b'0') as i32;
                }
                _ => {
                    match prev {
                        b'+' => v.push(n),
                        b'-' => v.push(-n),
                        b'*' => *v.last_mut().unwrap() *= n,
                        b'/' => *v.last_mut().unwrap() /= n,
                        _ => {}
                    }
                    prev = b;
                    n = 0;
                }
            }
        }
        v.iter().sum()
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "3+2*2".to_string();
        let ans = 7;
        assert_eq!(ans, Solution::calculate(s));
    }

    #[test]
    fn test_2() {
        let s = " 3/2 ".to_string();
        let ans = 1;
        assert_eq!(ans, Solution::calculate(s));
    }

    #[test]
    fn test_3() {
        let s = " 3+5 / 2".to_string();
        let ans = 5;
        assert_eq!(ans, Solution::calculate(s));
    }
}
