struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut arr = [0; 26];
        for &i in s.as_bytes() {
            arr[(i - 'a' as u8) as usize] += 1;
        }
        let mut ret = vec![];
        let mut hm = HashMap::new();
        let mut bg = 0;
        for (idx, &val) in s.as_bytes().iter().enumerate() {
            *hm.entry(val).or_insert(0) += 1;

            let mut partition = true;
            for (&char, &count) in hm.iter() {
                if count != arr[(char - 'a' as u8) as usize] {
                    partition = false;
                    break;
                }
            }
            if partition {
                ret.push(idx as i32 - bg + 1);
                bg = idx as i32 +1;
                hm.clear();
            }
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![9, 7, 8],
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![10],
            Solution::partition_labels("eccbbbbdec".to_string())
        );
    }
}
