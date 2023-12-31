struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn maximum_length(s: String) -> i32 {
        let table = Self::prepare_table(s);
        // println!("table: {:?}", table);
        table
            .into_iter()
            .map(|x| Self::calculate_a_character(&x))
            .max()
            .unwrap()
    }
    fn prepare_table(s: String) -> [Vec<usize>; 26] {
        let mut ret: [Vec<usize>; 26] = Default::default();
        for (i, c) in s.chars().enumerate() {
            ret[c as usize - 'a' as usize].push(i);
        }
        ret
    }

    fn calculate_a_character(v: &[usize]) -> i32 {
        if v.len() < 3 {
            return -1;
        }
        let mut start = v[0];
        let mut count = 1;
        let mut hm = std::collections::HashMap::<usize, usize>::new();
        for &val in v.iter().skip(1) {
            if val != start + count {
                for special_count in 1..=count {
                    *hm.entry(special_count).or_insert(0) += count - special_count + 1;
                }
                count = 1;
                start = val;
            } else {
                count += 1;
            }
        }
        for special_count in 1..=count {
            *hm.entry(special_count).or_insert(0) += count - special_count + 1;
        }
        // println!("hm: {:?}", hm);
        hm.into_iter()
            .filter(|(_spec_len, count)| *count >= 3)
            .map(|(spec_len, _)| spec_len as i32)
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let s = "aaaa".to_string();

        assert_eq!(2, Solution::maximum_length(s));
    }
    #[test]
    fn test_2() {
        let s = "abcaba".to_string();
        
        assert_eq!(1, Solution::maximum_length(s));
    }
}
