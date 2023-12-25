struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn calculate(s: String) -> i32 {
        let mut sign = 1;
        let mut cur = 0;
        let mut num = 0;
        let mut stack = vec![];
        let mut opers = vec![];
        for s in s.chars() {
            let n = s as i32 - '0' as i32;
            if n >= 0 && n < 10 {
                num = num * 10 + n;
            } else {
                cur += sign * num;
                match s {
                    '(' => {
                        opers.push(sign);
                        stack.push(cur);
                        sign = 1;
                        cur = 0;
                    }
                    ')' => {
                        cur *= opers.pop().unwrap();
                        cur += stack.pop().unwrap();
                    }
                    '+' | '-' => {
                        sign = if s == '+' { 1 } else { -1 };
                    }
                    _ => {
                        // ignore
                    }
                }
                num = 0;
            }
        }
        cur + num * sign
    }
}
