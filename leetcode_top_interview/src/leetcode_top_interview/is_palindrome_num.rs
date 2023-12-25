struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else if x == 0 {
            true
        } else if x % 10 == 0 {
            false
        } else {
            Self::inner(x)
        }
    }
    fn inner(mut x: i32) -> bool {
        let ori = x;
        let mut new = 0;
        while x > 0 {
            new *= 10;
            new += x % 10;
            x /= 10;
        }
        ori == new
    }
}
