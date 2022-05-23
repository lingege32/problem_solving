struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        fn str_to_zero_one(str: String) -> (i32, i32) {
            let (mut mm, mut nn) = (0, 0);
            for &s in str.as_bytes() {
                if s == '0' as u8 {
                    mm += 1;
                } else {
                    nn += 1;
                }
            }
            (mm, nn)
        }
        let all_set = strs
            .into_iter()
            .map(|x| str_to_zero_one(x))
            .collect::<Vec<_>>();
        let mut ans = 0;
        fn recur(all_set: &[(i32, i32)], cur: i32, ans: &mut i32, m: i32, n: i32) {
            if all_set.is_empty() || (cur + all_set.len() as i32) < *ans {
                return;
            }

            for start in 0..all_set.len() {
                let cur_set = all_set[start];
                if m >= cur_set.0 && n >= cur_set.1 {
                    *ans = (cur + 1).max(*ans);
                    recur(
                        &all_set[(start + 1)..],
                        cur + 1,
                        ans,
                        m - cur_set.0,
                        n - cur_set.1,
                    );
                }
            }
        }
        recur(&all_set, 0, &mut ans, m, n);
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let strs = ["10", "0001", "111001", "1", "0"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let (m, n) = (5, 3);
        let ans = 4;
        assert_eq!(ans, Solution::find_max_form(strs, m, n))
    }
    #[test]
    fn test_4() {
        let strs = ["10", "0001", "111001", "1", "0"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let (m, n) = (4, 3);
        let ans = 3;
        assert_eq!(ans, Solution::find_max_form(strs, m, n))
    }
    #[test]
    fn test_2() {
        let strs = ["10", "0", "1"].iter().map(|x| x.to_string()).collect();
        let (m, n) = (1, 1);
        let ans = 2;
        assert_eq!(ans, Solution::find_max_form(strs, m, n))
    }
}
