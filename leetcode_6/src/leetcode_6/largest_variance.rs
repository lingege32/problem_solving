struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn largest_variance(s: String) -> i32 {
        let s = {
            let mut array = Vec::with_capacity(s.len());
            let ss = s;
            ss.chars().for_each(|c| {
                array.push(c as usize - 'a' as usize);
            });
            array
        };
        let mut dict = [0; 26];
        for &i in &s {
            dict[i] += 1;
        }
        let mut ans = 0;
        for i in 0..25 {
            if dict[i] == 0 {
                continue;
            }
            for j in i + 1..26 {
                if dict[j] == 0 {
                    continue;
                }
                let val1 = Self::kadane(i, j, &s);
                let val2 = Self::kadane(j, i, &s);
                ans = ans.max(val1);
                ans = ans.max(val2);
            }
        }
        ans
    }
    fn kadane(i: usize, j: usize, s: &[usize]) -> i32 {
        let mut ans = 0;
        let mut diff = 0;
        let mut meet_j = 0;
        // aababbb
        for &c in s {
            if c == i {
                diff += 1;
            } else if c == j {
                diff -= 1;
                meet_j = 1;
            }

            ans = if meet_j == 1 {
                ans.max(diff)
            } else {
                ans.max(diff - 1)
            };
            if diff < 0 {
                // reset the y
                // aa bbb aa bb aaaaaaa
                //       | here diff = -1 and we reset to the a
                meet_j = 0;
                diff = 0;
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
        let s = "aababbb".to_string();
        let ans = 3;
        assert_eq!(ans, Solution::largest_variance(s));
    }

    #[test]
    fn test_2() {
        let s = "abcde".to_string();
        let ans = 0;
        assert_eq!(ans, Solution::largest_variance(s));
    }
}
