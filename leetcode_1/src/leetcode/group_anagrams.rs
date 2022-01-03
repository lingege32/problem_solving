struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::<[u8; 26], Vec<String>>::new();
        for str in strs {
            let mut k = [0; 26];
            str.as_bytes().iter().for_each(|x| {
                k[(*x - 'a' as u8) as usize] += 1;
            });
            let en = hm.entry(k).or_insert(Vec::new());
            en.push(str);
        }
        hm.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let mut ans = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        for i in ans.iter_mut() {
            i.sort();
        }
        ans.sort();
        let mut strs = Solution::group_anagrams(strs);
        for i in strs.iter_mut() {
            i.sort();
        }
        strs.sort();
        assert_eq!(strs, ans);
    }
}
