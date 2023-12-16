struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.into_bytes();
        let mut table = [usize::MAX; 256];
        let mut left = 0;
        let mut ret = 0;
        for (idx, &c) in s.iter().enumerate() {
            let i = c as usize;
            let old_index = table[i];
            if old_index != usize::MAX {
                ret = ret.max(idx - left);
                while left <= old_index {
                    table[s[left] as usize] = usize::MAX;
                    left += 1;
                }
            }
            table[i] = idx;
        }
        ret.max(s.len() - left) as i32
    }
}
