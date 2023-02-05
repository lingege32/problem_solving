struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ret = vec![];
        if s.len() < p.len() {
            return ret;
        }
        let p_len = p.len();
        let pattern = {
            let mut pat = [0; 26];
            p.into_bytes().into_iter().for_each(|x| {
                let idx = x - 'a' as u8;
                pat[idx as usize] += 1;
            });
            pat
        };
        let mut sss = [0; 26];
        let s = s.into_bytes();
        s[0..p_len].iter().for_each(|&x| {
            let idx = x - 'a' as u8;
            sss[idx as usize] += 1;
        });
        if sss == pattern {
            ret.push(0);
        }
        for idx in 0..s.len() - p_len {
            let head = s[idx] - 'a' as u8;
            let tail = s[idx + p_len] - 'a' as u8;
            sss[head as usize] -= 1;
            sss[tail as usize] += 1;
            if sss == pattern {
                ret.push(idx as i32 + 1);
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
        let s = "cbaebabacd".to_owned();
        let p = "abc".to_owned();
        let ans = vec![0,6];
        assert_eq!(ans, Solution::find_anagrams(s, p));
    }
}