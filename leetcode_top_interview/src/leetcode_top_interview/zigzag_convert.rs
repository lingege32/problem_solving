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

        vs.iter().map(|s| s.chars()).flatten().collect::<String>()
    }
}
