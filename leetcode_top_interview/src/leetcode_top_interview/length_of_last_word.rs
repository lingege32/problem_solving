struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.into_bytes();
        let first_not_space: usize = {
            let mut i = usize::MAX;
            for idx in (0..s.len()).rev() {
                if s[idx] != ' ' as u8 {
                    i = idx;
                    break;
                }
            }
            i
        };

        let space = {
            let mut i = usize::MAX;
            for idx in (0..first_not_space).rev() {
                if s[idx] == ' ' as u8 {
                    i = idx;
                    break;
                }
            }
            i
        };
        if space == usize::MAX {
            first_not_space as i32 + 1
        } else {
            (first_not_space - space) as i32
        }
    }
}
