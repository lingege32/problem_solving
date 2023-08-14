struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::<[u8; 26], Vec<String>>::new();
        for str in strs {
            let arr = Self::flat_to_array(&str);
            hm.entry(arr).or_default().push(str);
        }
        hm.into_iter().map(|(_, x)| x).collect()
    }

    fn flat_to_array(s: &str) -> [u8; 26] {
        let mut arr = [0; 26];
        for c in s.chars() {
            arr[c as usize - 'a' as usize] += 1;
        }
        arr
    }
}
