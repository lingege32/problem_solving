struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for c in s.chars() {
            if c == ']' || c == ')' || c == '}' {
                match stack.last().cloned() {
                    None => {
                        return false;
                    }
                    Some(back) => {
                        if (c == ']' && back != '[')
                            || (c == ')' && back != '(')
                            || (c == '}' && back != '{')
                        {
                            return false;
                        }
                        stack.pop();
                    }
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
