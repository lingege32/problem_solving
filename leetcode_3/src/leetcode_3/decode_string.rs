struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn decode_string(s: String) -> String {
        let s = s.as_bytes();
        fn inner(s: &[u8]) -> (String, usize) {
            if s.is_empty() {
                (String::new(), 0)
            } else {
                let mut idx = 0;
                let mut ans = String::new();
                while idx < s.len() {
                    // println!("s[{}] = {}, ans: {}", idx , s[idx] as char, ans);
                    match s[idx] as char {
                        'a'..='z' => {
                            let next = (idx + 1..s.len())
                                .find(|&x| match s[x] as char {
                                    'a'..='z' => false,
                                    _ => true,
                                })
                                .unwrap_or(s.len());
                            unsafe {
                                ans.push_str(std::str::from_utf8_unchecked(&s[idx..next]));
                            }
                            idx = next;
                        }
                        '0'..='9' => {
                            let next = (idx + 1..s.len())
                                .find(|&x| !matches!(s[x] as char, '0'..='9'))
                                .unwrap();
                            let n = unsafe {
                                std::str::from_utf8_unchecked(&s[idx..next])
                                    .parse::<i32>()
                                    .unwrap()
                            };
                            let (ss, bracket_width) = inner(&s[next + 1..]);
                            ans.reserve(n as usize * s.len());
                            for _ in 0..n {
                                ans.push_str(&ss);
                            }
                            // println!("idx: {}, next: {}, bracket_width: {}", idx, next, bracket_width);
                            idx = next + 1 + bracket_width;
                            // println!("s: {}, idx: {}", std::str::from_utf8(s).unwrap(), idx);
                        }
                        ']' => {
                            return (ans, idx + 1);
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                (ans, idx)
            }
        }
        inner(s).0
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "3[a]2[bc]".to_string();
        let ans = "aaabcbc".to_string();
        assert_eq!(ans, Solution::decode_string(s));
    }

    #[test]
    fn test_2() {
        let s = "3[a2[c]]".to_string();
        let ans = "accaccacc".to_string();
        assert_eq!(ans, Solution::decode_string(s));
    }

    #[test]
    fn test_3() {
        let s = "2[abc]3[cd]ef".to_string();
        let ans = "abcabccdcdcdef".to_string();
        assert_eq!(ans, Solution::decode_string(s));
    }
    #[test]
    fn test_4() {
        let s = "100[leetcode]".to_string();
        let ans = "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode".to_string();
        assert_eq!(ans, Solution::decode_string(s));
    }
}
