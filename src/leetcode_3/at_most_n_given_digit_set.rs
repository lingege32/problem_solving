struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n_digits = {
            let mut nb = n;
            let mut v = Vec::new();
            while nb != 0 {
                v.push(nb % 10);
                nb /= 10;
            }
            v
        };
        let digits = digits
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut dp = vec![0; n_digits.len() - 1];
        if !dp.is_empty() {
            dp[0] = digits.len() as i32;
            for i in 1..dp.len() {
                dp[i] = dp[i - 1] * digits.len() as i32;
            }
        }
        println!("dp: {:?}, digits: {:?}, n_digits: {:?}", dp, digits, n_digits);
        let mut ans = 0;
        let mut last_idx = n_digits.len() - 1;
        'outer: loop {
            let n = n_digits[last_idx];
            let mut finish = true;
            'inner: for &digit in digits.iter() {
                if digit < n {
                    ans += if last_idx > 0 { dp[last_idx - 1] } else { 1 };
                } else if digit == n {
                    if last_idx > 0 {
                        last_idx-=1;
                        finish = false;
                        break 'inner;
                    } else {
                        ans += 1;
                        break 'outer;
                    }
                } else {
                    break 'outer;
                }
            }
            if finish {
                break;
            }
        }
        for d in dp {
            ans += d;
        }
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let digits = ["1", "3", "5", "7"].iter().map(|x| x.to_string()).collect();
        let n = 100;
        let ans = 20;
        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), ans);
    }

    #[test]
    fn test_2() {
        let digits = ["1", "4", "9"].iter().map(|x| x.to_string()).collect();
        let n = 1000000000;
        let ans = 29523;
        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), ans);
    }

    #[test]
    fn test_3() {
        let digits = ["7"].iter().map(|x| x.to_string()).collect();
        let n = 8;
        let ans = 1;
        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), ans);
    }

    #[test]
    fn test_4() {
        let digits = ["5", "7", "8"].iter().map(|x| x.to_string()).collect();
        let n = 59;
        let ans = 6;
        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), ans);
    }

    #[test]
    fn test_5() {
        let digits = ["1"].iter().map(|x| x.to_string()).collect();
        let n = 834;
        let ans = 3;
        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), ans);
    }
}
