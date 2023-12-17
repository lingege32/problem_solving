struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        if s.len() % 2 != 0 {
            return false;
        }
        let mut stack = Vec::with_capacity(s.len() / 2);
        for c in s.chars() {
            match c {
                '{' | '(' | '[' => {
                    stack.push(c);
                }
                else_c => {
                    let corr = if else_c == '}' {
                        '{'
                    } else if else_c == ']' {
                        '['
                    } else {
                        '('
                    };
                    let top = stack.pop();
                    if !top.map_or(false, |x| x == corr) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
