struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let u8_path = path.as_bytes();
        let mut ans = vec![];
        for path in u8_path.split(|x| *x == '/' as u8) {
            if path.is_empty() || path == ".".as_bytes() {
                // do nothing
            } else if path == "..".as_bytes() {
                ans.pop();
            } else {
                ans.push(path);
            }
        }
        if ans.is_empty() {
            "/".to_string()
        } else {
            let mut ans2 = vec!["".as_bytes()];
            ans2.append(&mut ans);
            unsafe { String::from_utf8_unchecked(ans2.join(&('/' as u8))) }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let path = "/home/".to_string();
        let output = "/home".to_string();
        assert_eq!(output, Solution::simplify_path(path));
    }
    #[test]
    fn test_2() {
        let path = "/../".to_string();
        let output = "/".to_string();
        assert_eq!(output, Solution::simplify_path(path));
    }
    #[test]
    fn test_3() {
        let path = "/home//foo/".to_string();
        let output = "/home/foo".to_string();
        assert_eq!(output, Solution::simplify_path(path));
    }
    #[test]
    fn test_4() {
        let path = "/a/./b/../../c/".to_string();
        let output = "/c".to_string();
        assert_eq!(output, Solution::simplify_path(path));
    }
}
