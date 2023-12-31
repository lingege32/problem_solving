struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn maximum_length(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut min = -1;
        for sub_len in 1.. {
            let mut map = [0; 26];
            for window in s.windows(sub_len) {
                if Self::is_special(window) {
                    map[window[0] as usize - 'a' as usize] += 1;
                }
            }
            if map.iter().any(|x| *x >= 3) {
                min = sub_len as i32;
            } else {
                break;
            }
        }

        min
    }
    fn is_special(window: &[char]) -> bool {
        let c = window[0];
        window.iter().all(|x| *x == c)
    }
}
