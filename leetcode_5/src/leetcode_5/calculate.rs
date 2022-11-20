struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn calculate(s: String) -> i32 {
        let s = s.into_bytes();
        Self::inter_calculate(s)
    }
    fn inter_calculate(s: Vec<u8>) -> i32 {
        let mut sign: i32 = 1;
        let mut res: i32 = 0;
        let mut stack = Vec::new();
        let mut idx = 0;
        while idx < s.len() {
            let value = unsafe { *s.get_unchecked(idx) };
            if value >= '0' as u8 {
                let mut local_sum = (value - '0' as u8) as i32;
                'inner: loop {
                    idx += 1;
                    if idx == s.len() {
                        break 'inner;
                    }
                    let v2 = unsafe { *s.get_unchecked(idx) };
                    if v2 >= '0' as u8 {
                        local_sum = local_sum * 10 + (v2 - '0' as u8) as i32;
                    } else {
                        break 'inner;
                    }
                }
                idx -= 1;
                res += sign * local_sum;
            } else if value == '+' as u8 {
                sign = 1;
            } else if value == '-' as u8 {
                sign = -1;
            } else if value == '(' as u8 {
                stack.push(res);
                stack.push(sign);
                res = 0;
                sign = 1;
            } else if value == ')' as u8 {
                let osign = stack.pop().unwrap();
                let ores = stack.pop().unwrap();
                res = osign * res + ores;
            }
            idx += 1;
        }

        res
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "1 + 1".to_owned();
        let ans = Solution::calculate(s);
        assert_eq!(ans, 2);
    }
}
