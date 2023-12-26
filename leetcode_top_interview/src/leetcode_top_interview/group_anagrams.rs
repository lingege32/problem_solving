struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::<_, Vec<String>>::new();

        for str in strs.into_iter() {
            let arr = Self::encode(&str);
            hm.entry(arr).or_default().push(str);
        }

        hm.into_values().collect::<Vec<_>>()
    }
    fn encode(s: &str) -> [u8; 26] {
        let mut arr = [0u8; 26];
        for c in s.chars() {
            arr[c as usize - 'a' as usize] += 1;
        }
        arr
    }
}
