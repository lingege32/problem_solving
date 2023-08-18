struct Solution;

const LEN: usize = 'z' as usize - 'A' as usize + 1;

impl Solution {
    #[allow(dead_code)]
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_owned();
        }
        // s = "ABCDEFG"
        // t = "AE"
        // Then, left is 2
        // i is 0 and point to s[0], traverse the s by idx.


        // left means how many charaters in target is traversed
        let mut left = t.len();
        let mut i = 0;

        // t is the dict to store all target charaters counts
        let mut t = Self::prepare(t);
        let s: &[u8] = s.as_bytes();
        let mut range = None;
        for (idx, &val) in s.iter().enumerate() {
            // step 1.
            // traverse the idx, 
            // if t[val as usize - 'A' as usize] >= 0, it means that we traverse the charaters in target.
            t[val as usize - 'A' as usize] -= 1;
            if t[val as usize - 'A' as usize] >= 0 {
                left -= 1;
            }

            // we traverse all charaters in target if left == 0
            while left == 0 {
                // update the range if dis is less than before
                match range {
                    None => {
                        range = Some((i, idx));
                    }
                    Some(r) => {
                        let dis = r.1 - r.0;
                        if idx - i < dis {
                            range = Some((i, idx));
                        } else {
                            range = Some(r);
                        }
                    }
                }

                // update the left index 'i' to decrease the substring
                // if traverse the charater in target, the t[s[i] as usize - 'A' as usize] == 1 is raise.
                // then we need add left and find the next substring to move idx.
                t[s[i] as usize - 'A' as usize] += 1;
                if t[s[i] as usize - 'A' as usize] == 1 {
                    left += 1;
                }
                i += 1;
            }
        }
        match range {
            None => "".to_owned(),
            Some(r) => std::str::from_utf8(&s[r.0..=r.1]).unwrap().to_owned(),
        }
    }
    fn prepare(t: String) -> [i32; LEN] {
        let slice = t.as_bytes().iter().fold([0; LEN], |mut ret, u| {
            ret[*u as usize - 'A' as usize] += 1;
            ret
        });
        slice
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let ans = "BANC".to_string();
        assert_eq!(ans, Solution::min_window(s, t));
    }
    #[test]
    fn test_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let ans = "a".to_string();
        assert_eq!(ans, Solution::min_window(s, t));
    }
}
