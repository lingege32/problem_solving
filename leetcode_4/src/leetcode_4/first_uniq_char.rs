struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut table = vec![None; 26];
        for (idx, val) in s.as_bytes().iter().enumerate() {
            let c = (*val - 'a' as u8) as usize;
            let v = &mut table[c];
            match v {
                Some(_) => {
                    v.replace(usize::MAX);
                }
                None => {
                    *v = Some(idx);
                }
            }
        }
        table.iter().map(|x| x.unwrap_or(usize::MAX)).min().map_or(-1, |x| x as i32)
    }
}
