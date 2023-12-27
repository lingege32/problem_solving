struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0f64;
        }
        let n = n as i64;
        if n < 0 {
            1f64 / Self::my_positive_pow(x, -n)
        } else {
            Self::my_positive_pow(x, n)
        }
    }

    fn my_positive_pow(mut x: f64, mut n: i64) -> f64 {
        let mut power = 1.0;
        while n > 0 {
            if n % 2 == 1 {
                power *= x;
            }
            x *= x;
            n /= 2;
        }
        power
    }
}
