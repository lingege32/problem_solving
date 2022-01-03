struct Solution {}

// algorithm take O(slen+tlen)
impl Solution {
    #[allow(dead_code)]
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        // let s = s.as_bytes().iter().map(|&x| x as usize).collect::<Vec<_>>();
        // let t = t.as_bytes().iter().map(|&x| x as usize).collect::<Vec<_>>();
        let (slen, tlen) = (s.len(), t.len());
        if tlen > slen {
            String::new()
        } else {
            let mut freq = [0; 52];
            fn get_ref<'a>(c: u8, freq: &'a mut [i32; 52]) -> &'a mut i32 {
                let idx = if c >= 'a' as u8 {
                    26 + (c - 'a' as u8) as usize
                } else {
                    (c - 'A' as u8) as usize
                };
                &mut freq[idx]
            }
            for c in t {
                *get_ref(*c, &mut freq) += 1;
            }
            let mut max = usize::MAX;
            let mut start = 0;
            let (mut i, mut left) = (0, tlen);
            for j in 0..slen {
                let rsj = get_ref(s[j], &mut freq);
                *rsj -= 1;
                if *rsj >= 0 {
                    left -= 1;
                }
                while left == 0 {
                    if j - i + 1 < max {
                        max = j - i + 1;
                        start = i;
                    }
                    let rsi = get_ref(s[i], &mut freq);

                    *rsi += 1;
                    if *rsi == 1 {
                        left += 1;
                    }
                    i += 1;
                }
            }
            if max == usize::MAX {
                String::new()
            } else {
                let a = s[start..start + max]
                    .iter()
                    .map(|x| *x as u8)
                    .collect::<Vec<_>>();
                String::from_utf8(a).unwrap()
            }
        }
    }
}

// O(s.len() * t.len()) bad
// impl Solution {
//     #[allow(dead_code)]
//     pub fn min_window(s: String, t: String) -> String {
//         #[inline(always)]
//         fn array_cmp(a: &[u8; 52], b: &[u8; 52]) -> bool {
//             for idx in 0..52 {
//                 if a[idx] != b[idx] {
//                     return false;
//                 }
//             }
//             true
//         }
//         #[inline(always)]
//         fn u8_to_usize(char: u8) -> usize {
//             if char >= 'a' as u8 {
//                 26 + (char - 'a' as u8) as usize
//             } else {
//                 (char - 'A' as u8) as usize
//             }
//         }

//         let s = s.as_bytes();
//         let t = t.as_bytes();
//         let slen = s.len();
//         let tlen = t.len();
//         if s.len() < t.len() {
//             String::new()
//         } else {
//             let mut pattern = [0; 52];
//             for i in t {
//                 pattern[u8_to_usize(*i)] += 1;
//             }
//             let is_in_pattern = |x: u8| -> bool { pattern[u8_to_usize(x)] != 0 };
//             let append_c = |x: u8, arr: &mut [u8; 52]| {
//                 if is_in_pattern(x) {
//                     let idx = u8_to_usize(x);
//                     if pattern[idx] > arr[idx] {
//                         arr[idx] += 1;
//                     }
//                 }
//             };
//             let mut dp = vec![(0usize..=s.len() - t.len())
//                 .map(|x| {
//                     let mut ans = [0; 52];
//                     for c in &s[x..x + t.len()] {
//                         if is_in_pattern(*c) {
//                             append_c(*c, &mut ans);
//                         }
//                     }
//                     ans
//                 })
//                 .collect::<Vec<_>>()];

//             for (idx, i) in dp[0].iter().enumerate() {
//                 if array_cmp(i, &pattern) {
//                     return String::from_utf8_lossy(&s[idx..idx + t.len()]).to_string();
//                 }
//             }
//             for dp_idx in 1..=slen - tlen {
//                 let window_width = dp_idx + tlen;
//                 dp.push(vec![]);
//                 for start in 0..=(slen - window_width) {
//                     let mut last_solution = dp[dp_idx - 1][start];
//                     let c = s[start + window_width - 1];
//                     append_c(c, &mut last_solution);
//                     if array_cmp(&last_solution, &pattern) {
//                         return String::from_utf8_lossy(&s[start..start + window_width])
//                             .to_string();
//                     }
//                     dp[dp_idx].push(last_solution);
//                 }
//             }
//             String::new()
//         }
//     }
// }

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

    #[test]
    fn test_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let ans = "".to_string();
        assert_eq!(ans, Solution::min_window(s, t));
    }

    #[test]
    fn test_4() {
        let s = "aaaaaaaaaaaabbbbbcdd".to_string();
        let t = "abcdd".to_string();
        let ans = "abbbbbcdd".to_string();
        assert_eq!(ans, Solution::min_window(s, t));
    }
}
