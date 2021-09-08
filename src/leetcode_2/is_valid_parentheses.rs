struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.as_bytes() {
            match *c as char{
                '[' | '(' | '{'=> {
                    stack.push(c);                    
                },
                else_c => {
                    let corr = if else_c == '}' {
                        '{'
                    } else if else_c == ']' {
                        '['
                    } else {
                        '('
                    };
                    let top = stack.pop();
                    if !top.map_or(false, |x| *x == corr as u8) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}


#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "()".to_string();
        assert!(Solution::is_valid(s));
    }
    #[test]
    fn test_2() {
        let s = "()[]{}".to_string();
        assert!(Solution::is_valid(s));
    }
    #[test]
    fn test_3() {
        let s = "(]".to_string();
        assert!(!Solution::is_valid(s));
    }
    #[test]
    fn test_4() {
        let s = "([)]".to_string();
        assert!(!Solution::is_valid(s));
    }
    #[test]
    fn test_5() {
        let s = "{[]}".to_string();
        assert!(Solution::is_valid(s));
    }
}