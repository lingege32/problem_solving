struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut vs = vec![String::new(); num_rows];
        let num_rows = num_rows - 1;
        let mut idx = 0i64;
        let mut shift = 1i64;
        for c in s.chars() {
            vs[idx as usize].push(c);
            if idx == 0 {
                shift = 1;
            } else if idx == num_rows as i64 {
                shift = -1;
            }
            idx += shift;
        }
        let mut ret = String::new();
        for idx in vs.into_iter() {
            ret.push_str(&idx);
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "PAYPALISHIRING".to_owned();
        let num_rows = 3;
        let ans = "PAHNAPLSIIGYIR".to_owned();
        assert_eq!(ans, Solution::convert(s, num_rows));
    }
    #[test]
    fn test_2() {
        let s = "A".to_owned();
        let num_rows = 1;
        let ans = "A".to_owned();
        assert_eq!(ans, Solution::convert(s, num_rows));
    }
    
    #[test]
    fn test_3() {
        let s = "PAYPALISHIRING".to_owned();
        let num_rows = 4;
        let ans = "PINALSIGYAHRPI".to_owned();
        assert_eq!(ans, Solution::convert(s, num_rows));
    }
}
