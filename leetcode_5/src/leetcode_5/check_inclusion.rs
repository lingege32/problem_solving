struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let mut pattern = [0; 26];
        for c in &s1 {
            pattern[(*c - 'a' as u8) as usize] += 1;
        }
        let mut target = [0;26];
        for c in &s2[0..s1.len()] {
            target[(*c - 'a' as u8) as usize] += 1;
        }
        if target == pattern {
            return true;
        }

        for idx in 0..(s2.len() - s1.len()) {
            let head = (s2[idx] - 'a' as u8) as usize;
            let tail = (s2[idx+s1.len()] - 'a' as u8) as usize;
            target[head] -= 1;
            target[tail] += 1;
            if target == pattern {
                 return true;
            }
        }
        false
    }
}
