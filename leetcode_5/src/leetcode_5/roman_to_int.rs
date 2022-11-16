struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let v = s.into_bytes();
        let mut ret = 0;
        let mut idx = 0;
        while idx < v.len() {
            let n = v.get(idx + 1).unwrap_or(&0);
            match *unsafe { v.get_unchecked(idx) } as char {
                'I' => {
                    if *n == 'V' as u8 {
                        ret += 4;
                        idx += 1;
                    } else if *n == 'X' as u8 {
                        ret += 9;
                        idx += 1;
                    } else {
                        ret += 1;
                    }
                }
                'V' => {
                    ret += 5;
                }
                'X' => {
                    if *n == 'L' as u8 {
                        ret += 40;
                        idx += 1;
                    } else if *n == 'C' as u8 {
                        ret += 90;
                        idx += 1;
                    } else {
                        ret += 10;
                    }
                }
                'L' => {
                    ret += 50;
                }
                'C' => {
                    if *n == 'D' as u8 {
                        ret += 400;
                        idx += 1;
                    } else if *n == 'M' as u8 {
                        ret += 900;
                        idx += 1;
                    } else {
                        ret += 100;
                    }
                }
                'D' => {
                    ret += 500;
                }
                'M' => {
                    ret += 1000;
                }
                _ => {}
            }
            idx += 1;
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = Solution::roman_to_int("MCMXCIV".to_owned());
        assert_eq!(1994, a);
    }
}