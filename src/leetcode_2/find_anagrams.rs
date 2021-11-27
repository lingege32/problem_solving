struct Solution {
    
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut vs = [0;26];
        let mut vp = [0;26];
        p.as_bytes().iter().for_each(|&x| vp[(x-'a' as u8) as usize]+=1);
        let l = p.len();
        let mut ans = Vec::new();
        for (idx, &val) in s.as_bytes().iter().enumerate() {
            if idx>=l {
                let minus_bytes = s.as_bytes()[idx-l];
                vs[(minus_bytes - 'a' as u8) as usize] -= 1;
            }
            vs[(val - 'a' as u8) as usize] +=1;
            if vs==vp {
                ans.push((idx + 1 - l) as i32);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        assert_eq!(vec![0,6], Solution::find_anagrams(s, p));
    }

    #[test]
    fn test_2() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        assert_eq!(vec![0,1,2], Solution::find_anagrams(s, p));
    }


}