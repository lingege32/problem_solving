struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s = s;
        let mut t = t;
        fn inner(slice_u8: &mut [u8]) -> &[u8] {
            let mut cur_idx = 0;
            for idx in 0..slice_u8.len() {
                if slice_u8[idx] == '#' as u8 {
                    if cur_idx > 0 {
                        cur_idx -= 1;
                    }
                } else {
                    slice_u8[cur_idx] = slice_u8[idx];
                    cur_idx += 1;
                }
            }
            &slice_u8[0..cur_idx]
        }
        unsafe {
            let su8 = s.as_bytes_mut();
            let tu8 = t.as_bytes_mut();
            inner(su8) == inner(tu8)
        }
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        assert!(Solution::backspace_compare(s, t));
    }
    #[test]
    fn test_2() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        assert!(Solution::backspace_compare(s, t));
    }
    #[test]
    fn test_3() {
        let s = "a#c".to_string();
        let t = "b".to_string();
        assert!(!Solution::backspace_compare(s, t));
    }
}
