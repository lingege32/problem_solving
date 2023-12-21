struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for dir in path.split('/') {
            if dir.is_empty() {
                continue;
            }
            match dir {
                ".." => {
                    stack.pop();
                }
                "" | "." => {
                    // do nothing
                }
                d => {
                    stack.push(d);
                }
            }
        }

        format!("/{}", stack.join("/"))
    }
}
