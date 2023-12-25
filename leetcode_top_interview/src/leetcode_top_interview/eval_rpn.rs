struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for s in tokens.into_iter() {
            match s.as_ref() {
                "+" => {
                    let a = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += a;
                }
                "-" => {
                    let a = stack.pop().unwrap();
                    *stack.last_mut().unwrap() -= a;
                }
                "*" => {
                    let a = stack.pop().unwrap();
                    *stack.last_mut().unwrap() *= a;
                }
                "/" => {
                    let a = stack.pop().unwrap();
                    *stack.last_mut().unwrap() /= a;
                }
                s => {
                    stack.push(s.parse::<i32>().unwrap());
                }
            }
        }
        *stack.last().unwrap()
    }
}
