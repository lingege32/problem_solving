struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![None];
        let mut max = 0;
        for (idx, &c) in s.as_bytes().iter().enumerate() {
            if c == '(' as u8 {
                stack.push(Some(idx));
            } else {
                let _ = stack.pop();
                if stack.is_empty() {
                    stack.push(Some(idx));
                } else {
                    max = max.max(
                        stack
                            .last()
                            .unwrap()
                            .map_or((1 + idx) as i32, |last_idx| (idx - last_idx) as i32),
                    );
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "()".to_string();
        assert_eq!(2, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_2() {
        let s = ")()())".to_string();
        assert_eq!(4, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_3() {
        let s = "".to_string();
        assert_eq!(0, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_4() {
        let s = "()(()".to_string();
        assert_eq!(2, Solution::longest_valid_parentheses(s));
    }
}
