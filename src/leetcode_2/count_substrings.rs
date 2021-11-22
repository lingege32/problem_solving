struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let vec_u8 = s.as_bytes();
        let mut count = vec_u8.len() as i32;

        for idx in 1..vec_u8.len() - 1 {
            let mut beg = (idx - 1) as i32;
            let mut end = (idx + 1) as i32;
            while beg >= 0
                && (end as usize) < vec_u8.len()
                && vec_u8[beg as usize] == vec_u8[end as usize]
            {
                beg -= 1;
                end += 1;
                count += 1;
            }
        }
        for idx in 0..vec_u8.len() - 1 {
            let mut beg = idx as i32;
            let mut end = (idx + 1) as i32;
            while beg >= 0
                && (end as usize) < vec_u8.len()
                && vec_u8[beg as usize] == vec_u8[end as usize]
            {
                beg -= 1;
                end += 1;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_substrings("abc".to_string()));
    }
    #[test]
    fn test_2() {
        assert_eq!(6, Solution::count_substrings("aaa".to_string()));
    }
}
