struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut a_to_b = [Option::<u8>::None; 255];
        let mut b_to_a = [Option::<u8>::None; 255];
        for (a, b) in s.into_iter().zip(t.into_iter()) {
            match a_to_b[a as usize] {
                Some(c) => {
                    if b != c {
                        return false;
                    }
                }
                None => {
                    a_to_b[a as usize] = Some(b);
                }
            }
            match b_to_a[b as usize] {
                Some(c) => {
                    if a != c {
                        return false;
                    }
                }
                None => {
                    b_to_a[b as usize] = Some(a);
                }
            }
        }
        true
    }
}
