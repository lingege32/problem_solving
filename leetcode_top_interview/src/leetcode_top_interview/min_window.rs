struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_window(s: String, t: String) -> String {
        let m = s.len();
        let n = t.len();
        if m < n {
            return "".to_string();
        }

        let table = Self::prepare_table(t);
        let mut left = n;
        let mut counter: [i32; 52] = [0; 52];
        let new_s = s.chars().map(|c| Self::map_to_idx(c)).collect::<Vec<_>>();
        let mut left_idx = 0;
        let mut min = m + 1;
        let (mut min_left, mut min_right) = (0, 0);
        for (right, &idx) in new_s.iter().enumerate() {
            counter[idx] += 1;
            if counter[idx] <= table[idx] {
                left -= 1;
            }
            while left == 0 {
                let l_v = new_s[left_idx];
                counter[l_v] -= 1;
                if counter[l_v] < table[l_v] {
                    let length = right + 1 - left_idx;
                    if length < min {
                        min = length;
                        min_right = right + 1;
                        min_left = left_idx;
                    }
                    left += 1;
                }
                left_idx += 1;
            }
        }
        if min <= m {
            Self::to_str(&new_s[min_left..min_right])
        } else {
            "".to_string()
        }
    }

    fn prepare_table(t: String) -> [i32; 52] {
        let mut table = [0; 52];
        for c in t.chars() {
            table[Self::map_to_idx(c)] += 1;
        }
        table
    }
    fn map_to_idx(c: char) -> usize {
        match c {
            'a'..='z' => 26 + c as usize - 'a' as usize,
            'A'..='Z' => c as usize - 'A' as usize,
            _ => {
                std::unreachable!()
            }
        }
    }

    fn to_str(s: &[usize]) -> String {
        s.iter()
            .map(|&x| {
                if x > 25 {
                    ('a' as usize + x - 26) as u8 as char
                } else {
                    ('A' as usize + x) as u8 as char
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "ADOBECODEBANC".to_string();

        let t = "ABC".to_string();

        let ans = "BANC";
        assert_eq!(ans, Solution::min_window(s, t));
    }
    #[test]
    fn test_2() {
        let s = "ab".to_string();

        let t = "b".to_string();

        let ans = "b";
        assert_eq!(ans, Solution::min_window(s, t));
    }
    #[test]
    fn test_3() {
        let s = "b".to_string();

        let t = "b".to_string();

        let ans = "b";
        assert_eq!(ans, Solution::min_window(s, t));
    }
}
