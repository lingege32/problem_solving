struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut record = [0i32; 26];
        for w in s.chars() {
            record[w as usize - 'a' as usize] += 1;
        }
        for w in t.chars() {
            let r = record.get_mut(w as usize - 'a' as usize).unwrap();
            if *r == 0 {
                return false;
            }
            *r -= 1;
        }
        true
    }
}
